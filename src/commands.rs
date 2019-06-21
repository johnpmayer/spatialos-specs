use crate::component_registry::ComponentRegistry;
use crate::entities::EntityId;
use crate::storage::SpatialUnprotectedStorage;
use crate::ValueWithSystemData;
use spatialos_sdk::worker::commands::{IncomingCommandRequest, OutgoingCommandRequest};
use spatialos_sdk::worker::component::Component as WorkerComponent;
use spatialos_sdk::worker::connection::{Connection, WorkerConnection};
use spatialos_sdk::worker::op::{
    CommandResponse as WorkerCommandResponse, CommandResponseOp, StatusCode,
};
use spatialos_sdk::worker::RequestId;
use specs::prelude::{
    Component, Entities, Entity, HashMapStorage, Join, Resources, SystemData, Write, WriteStorage,
};
use std::collections::HashMap;

/// A storage which contains command requests for a given component
/// that have not been responded to yet.
///
/// You can respond to the requests via the `respond` method, for example:
///
/// ```ignore
/// impl<'a> System<'a> for PlayerCreatorSys {
///     type SystemData = (CommandRequests<'a, Player>, SpatialWriteStorage<'a, Player>);
///
///     fn run(&mut self, (mut requests, mut player): Self::SystemData) {
///         for (request, player) in (&mut requests, &mut player).join() {
///             request.respond(|request, caller_worker_id, _| match request {
///                 PlayerCommandRequest::UpdateHealth(request) => {
///                     player.health -= request.damage;
///
///                     Some(PlayerCommandResponse::UpdateHealth(
///                         UpdateHealthResponse {
///                             new_health: player.health
///                         },
///                     ))
///                 }
///             });
///         }
///     }
/// }
/// ```
///
/// If the closure given to `respond` returns `None`, the command request will
/// not be responded to. Please note that a command request will stay in this
/// component until it has been responded too.
///
/// A command will only be responded to in a single system. If `SysA` runs before
/// `SysB` and `SysB` responds to a request, `SysB` cannot see that request.
///
/// Asynchronous command responses are not yet supported.
///
pub type CommandRequests<'a, T> = WriteStorage<'a, CommandRequestsComp<T>>;

pub struct CommandRequestsComp<T: WorkerComponent> {
    requests: Vec<(
        RequestId<IncomingCommandRequest>,
        T::CommandRequest,
        String,
        Vec<String>,
    )>,
    responses: Vec<(RequestId<IncomingCommandRequest>, T::CommandResponse)>,
}

impl<T: WorkerComponent> Default for CommandRequestsComp<T> {
    fn default() -> Self {
        CommandRequestsComp {
            requests: Vec::new(),
            responses: Vec::new(),
        }
    }
}

impl<T: 'static + WorkerComponent> Component for CommandRequestsComp<T> {
    type Storage = SpatialUnprotectedStorage<T, Self, HashMapStorage<Self>>;
}

impl<T: 'static + WorkerComponent> CommandRequestsComp<T> {
    pub(crate) fn on_request(
        &mut self,
        request_id: RequestId<IncomingCommandRequest>,
        request: T::CommandRequest,
        caller_worker_id: String,
        caller_attribute_set: Vec<String>,
    ) {
        self.requests
            .push((request_id, request, caller_worker_id, caller_attribute_set));
    }

    /// Respond to the pending command requests.
    ///
    /// The given closure accepts a command request object and returns:
    ///
    /// * `Some(response)` to respond to the command.
    /// * `None` to not respond to the command, leaving the request for other systems or
    ///   the next frame.
    pub fn respond<F>(&mut self, mut responder: F)
    where
        F: FnMut(&T::CommandRequest, &String, &Vec<String>) -> Option<T::CommandResponse>,
    {
        let mut requests_left = Vec::new();
        for (request_id, request, caller_worker_id, caller_attribute_set) in self.requests.drain(..)
        {
            match responder(&request, &caller_worker_id, &caller_attribute_set) {
                Some(response) => self.responses.push((request_id, response)),
                None => requests_left.push((
                    request_id,
                    request,
                    caller_worker_id,
                    caller_attribute_set,
                )),
            }
        }

        self.requests = requests_left;
    }

    pub(crate) fn flush_responses(&mut self, connection: &mut WorkerConnection) {
        for (request_id, response) in self.responses.drain(..) {
            connection.send_command_response::<T>(request_id, response);
        }
    }
}

pub(crate) trait CommandRequestsExt {
    fn clear_empty_request_objects(&mut self, res: &Resources);
}

impl<'a, T: 'static + WorkerComponent> CommandRequestsExt for CommandRequests<'a, T> {
    fn clear_empty_request_objects(&mut self, res: &Resources) {
        let non_empty_requests: Vec<(CommandRequestsComp<T>, Entity)> =
            (self.drain(), &Entities::fetch(res))
                .join()
                .filter(|r| r.0.requests.len() > 0)
                .collect();

        self.clear();

        for (requests, entity) in non_empty_requests {
            self.insert(entity, requests)
                .expect("Error inserting command request object.");
        }
    }
}

type CommandResponse<'a, T> = ValueWithSystemData<
    'a,
    Result<&'a <T as WorkerComponent>::CommandResponse, StatusCode<WorkerCommandResponse<'a>>>,
>;

pub type CommandSender<'a, T> = Write<'a, CommandSenderRes<T>>;

type CommandIntermediateCallback = Box<FnOnce(&Resources, CommandResponseOp) + Send + Sync>;

pub struct CommandSenderRes<T: WorkerComponent> {
    callbacks: HashMap<RequestId<OutgoingCommandRequest>, CommandIntermediateCallback>,
    buffered_requests: Vec<(EntityId, T::CommandRequest, CommandIntermediateCallback)>,
}

impl<T: 'static + WorkerComponent> CommandSenderRes<T> {
    pub fn send_command<F>(&mut self, entity_id: EntityId, request: T::CommandRequest, callback: F)
    where
        F: 'static + FnOnce(CommandResponse<T>) + Send + Sync,
    {
        self.buffered_requests.push((
            entity_id,
            request,
            Box::new(|res, response_op| match response_op.response {
                StatusCode::Success(response) => {
                    let response = response.get::<T>().unwrap();
                    callback(CommandResponse::<T> {
                        res,
                        value: Ok(response),
                    })
                }
                other => callback(CommandResponse::<T> {
                    res,
                    value: Err(other),
                }),
            }),
        ));
    }

    pub(crate) fn got_command_response(res: &Resources, response_op: CommandResponseOp) {
        let callback = {
            CommandSender::<T>::fetch(res)
                .callbacks
                .remove(&response_op.request_id)
        };

        match callback {
            Some(callback) => callback(res, response_op),
            None => println!("Unknown request ID: {:?}", response_op.request_id),
        }
    }

    pub(crate) fn flush_requests(&mut self, connection: &mut WorkerConnection) {
        for (entity_id, request, callback) in self.buffered_requests.drain(..) {
            // TODO: Default command params like timeout
            let request_id = connection.send_command_request::<T>(
                entity_id.id(),
                request,
                None,
                Default::default(),
            );
            self.callbacks.insert(request_id, callback);
        }
    }
}

impl<T: 'static + WorkerComponent> Default for CommandSenderRes<T> {
    fn default() -> Self {
        ComponentRegistry::register_component::<T>();
        CommandSenderRes {
            callbacks: HashMap::new(),
            buffered_requests: Vec::new(),
        }
    }
}
