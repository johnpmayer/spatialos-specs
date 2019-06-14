use crate::component_registry::ComponentRegistry;
use crate::spatial_reader::ResourcesSystemData;
use crate::system_commands::SystemCommandSender;
use spatialos_sdk::worker::connection::WorkerConnection;
use specs::prelude::{System, WriteExpect};

/// A system which replicates changes in the local world to SpatialOS.
///
/// This system should run at the end of each frame.
///
/// This system **must not run in parallel with other systems**, or you may
/// get a runtime panic. You can ensure this by creating a barrier before the system.
///
/// ## Example
///
/// ```
/// let mut world = World::new();
///
/// let mut dispatcher = DispatcherBuilder::new()
///     .with(SpatialReaderSystem, "reader", &[])
///     .with_barrier()
///
///     .with(MovePlayerSys, "", &[])
///
///     .with_barrier()
///     .with(SpatialWriterSystem, "writer", &[])
///     .build();
///
/// dispatcher.setup(&mut world.res);
/// ```
pub struct SpatialWriterSystem;

impl<'a> System<'a> for SpatialWriterSystem {
    type SystemData = (
        WriteExpect<'a, WorkerConnection>,
        SystemCommandSender<'a>,
        ResourcesSystemData<'a>,
    );

    fn run(&mut self, (mut connection, mut system_command_sender, res): Self::SystemData) {
        for interface in res.res.fetch::<ComponentRegistry>().interfaces_iter() {
            interface.replicate(&res.res, &mut connection);
        }

        system_command_sender.flush_requests(&mut connection);
    }
}
