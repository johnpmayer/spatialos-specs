#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_mut)]

use spatialos_sdk::worker::internal::schema::*;
use spatialos_sdk::worker::component::*;
use std::collections::BTreeMap;

use super::generated as generated;

/* Enums. */
/* Types. */
/* Components. */ 


pub mod game {
use spatialos_sdk::worker::internal::schema::*;
use spatialos_sdk::worker::component::*;
use std::collections::BTreeMap;

use super::super::generated as generated;

/* Enums. */
/* Types. */
#[derive(Debug, Clone)]
pub struct CreatePlayerRequest {
    pub name: String,
}
impl TypeConversion for CreatePlayerRequest {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            name: input.field::<SchemaString>(1).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaString>(1).add(&&input.name);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CreatePlayerResponse {
}
impl TypeConversion for CreatePlayerResponse {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        Ok(())
    }
}

/* Components. */ 
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub current_direction: u32,
}
impl TypeConversion for Player {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            name: input.field::<SchemaString>(1).get_or_default(),
            current_direction: input.field::<SchemaUint32>(2).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaString>(1).add(&&input.name);
        output.field::<SchemaUint32>(2).add(input.current_direction);
        Ok(())
    }
}
impl ComponentData<Player> for Player {
    fn merge(&mut self, update: PlayerUpdate) {
        if let Some(value) = update.name { self.name = value; }
        if let Some(value) = update.current_direction { self.current_direction = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerUpdate {
    pub name: Option<String>,
    pub current_direction: Option<u32>,
}
impl TypeConversion for PlayerUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            name: None,
            current_direction: None,
        };
        let _field_name = input.field::<SchemaString>(1);
        if _field_name.count() > 0 {
            let field = &_field_name;
            output.name = Some(field.get_or_default());
        }
        let _field_current_direction = input.field::<SchemaUint32>(2);
        if _field_current_direction.count() > 0 {
            let field = &_field_current_direction;
            output.current_direction = Some(field.get_or_default());
        }
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.name {
            output.field::<SchemaString>(1).add(&value);
        }
        if let Some(value) = input.current_direction {
            output.field::<SchemaUint32>(2).add(value);
        }
        Ok(())
    }
}
impl ComponentUpdate<Player> for PlayerUpdate {
    fn merge(&mut self, update: PlayerUpdate) {
        if update.name.is_some() { self.name = update.name; }
        if update.current_direction.is_some() { self.current_direction = update.current_direction; }
    }
}

#[derive(Debug, Clone)]
pub enum PlayerCommandRequest {
}

#[derive(Debug, Clone)]
pub enum PlayerCommandResponse {
}

impl Component for Player {
    type Update = generated::game::PlayerUpdate;
    type CommandRequest = generated::game::PlayerCommandRequest;
    type CommandResponse = generated::game::PlayerCommandResponse;

    const ID: ComponentId = 1002;

    fn from_data(data: &SchemaComponentData) -> Result<generated::game::Player, String> {
        <generated::game::Player as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::game::PlayerUpdate, String> {
        <generated::game::PlayerUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::game::PlayerCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Player.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::game::PlayerCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Player.", command_index))
        }
    }

    fn to_data(data: &generated::game::Player) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::game::Player as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::game::PlayerUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::game::PlayerUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::game::PlayerCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::game::PlayerCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::game::PlayerCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::game::PlayerCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Player>());

#[derive(Debug, Clone)]
pub struct PlayerCreator {
}
impl TypeConversion for PlayerCreator {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        Ok(())
    }
}
impl ComponentData<PlayerCreator> for PlayerCreator {
    fn merge(&mut self, update: PlayerCreatorUpdate) {
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerCreatorUpdate {
}
impl TypeConversion for PlayerCreatorUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
        };
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        Ok(())
    }
}
impl ComponentUpdate<PlayerCreator> for PlayerCreatorUpdate {
    fn merge(&mut self, update: PlayerCreatorUpdate) {
    }
}

#[derive(Debug, Clone)]
pub enum PlayerCreatorCommandRequest {
    CreatePlayer(generated::game::CreatePlayerRequest),
}

#[derive(Debug, Clone)]
pub enum PlayerCreatorCommandResponse {
    CreatePlayer(generated::game::CreatePlayerResponse),
}

impl Component for PlayerCreator {
    type Update = generated::game::PlayerCreatorUpdate;
    type CommandRequest = generated::game::PlayerCreatorCommandRequest;
    type CommandResponse = generated::game::PlayerCreatorCommandResponse;

    const ID: ComponentId = 1001;

    fn from_data(data: &SchemaComponentData) -> Result<generated::game::PlayerCreator, String> {
        <generated::game::PlayerCreator as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::game::PlayerCreatorUpdate, String> {
        <generated::game::PlayerCreatorUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::game::PlayerCreatorCommandRequest, String> {
        match command_index {
            1 => {
                let result = <generated::game::CreatePlayerRequest as TypeConversion>::from_type(&request.object());
                result.and_then(|deserialized| Ok(PlayerCreatorCommandRequest::CreatePlayer(deserialized)))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component PlayerCreator.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::game::PlayerCreatorCommandResponse, String> {
        match command_index {
            1 => {
                let result = <generated::game::CreatePlayerResponse as TypeConversion>::from_type(&response.object());
                result.and_then(|deserialized| Ok(PlayerCreatorCommandResponse::CreatePlayer(deserialized)))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component PlayerCreator.", command_index))
        }
    }

    fn to_data(data: &generated::game::PlayerCreator) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::game::PlayerCreator as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::game::PlayerCreatorUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::game::PlayerCreatorUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::game::PlayerCreatorCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            PlayerCreatorCommandRequest::CreatePlayer(ref data) => {
                <generated::game::CreatePlayerRequest as TypeConversion>::to_type(data, &mut serialized_request.object_mut())?;
            },
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::game::PlayerCreatorCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            PlayerCreatorCommandResponse::CreatePlayer(ref data) => {
                <generated::game::CreatePlayerResponse as TypeConversion>::to_type(data, &mut serialized_response.object_mut())?;
            },
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::game::PlayerCreatorCommandRequest) -> u32 {
        match request {
            PlayerCreatorCommandRequest::CreatePlayer(_) => 1,
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::game::PlayerCreatorCommandResponse) -> u32 {
        match response {
            PlayerCreatorCommandResponse::CreatePlayer(_) => 1,
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<PlayerCreator>());


}

pub mod improbable {
use spatialos_sdk::worker::internal::schema::*;
use spatialos_sdk::worker::component::*;
use std::collections::BTreeMap;

use super::super::generated as generated;

/* Enums. */
/* Types. */
#[derive(Debug, Clone)]
pub struct ComponentInterest {
    pub queries: Vec<generated::improbable::ComponentInterest_Query>,
}
impl TypeConversion for ComponentInterest {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            queries: { let size = input.field::<SchemaObject>(1).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(<generated::improbable::ComponentInterest_Query as TypeConversion>::from_type(&input.field::<SchemaObject>(1).index(i))?); }; l },
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        for element in (&input.queries).iter() { <generated::improbable::ComponentInterest_Query as TypeConversion>::to_type(&element, &mut output.field::<SchemaObject>(1).add())?; };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_BoxConstraint {
    pub center: generated::improbable::Coordinates,
    pub edge_length: generated::improbable::EdgeLength,
}
impl TypeConversion for ComponentInterest_BoxConstraint {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            center: <generated::improbable::Coordinates as TypeConversion>::from_type(&input.field::<SchemaObject>(1).get_or_default())?,
            edge_length: <generated::improbable::EdgeLength as TypeConversion>::from_type(&input.field::<SchemaObject>(2).get_or_default())?,
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        <generated::improbable::Coordinates as TypeConversion>::to_type(&&input.center, &mut output.field::<SchemaObject>(1).add())?;
        <generated::improbable::EdgeLength as TypeConversion>::to_type(&&input.edge_length, &mut output.field::<SchemaObject>(2).add())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_CylinderConstraint {
    pub center: generated::improbable::Coordinates,
    pub radius: f64,
}
impl TypeConversion for ComponentInterest_CylinderConstraint {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            center: <generated::improbable::Coordinates as TypeConversion>::from_type(&input.field::<SchemaObject>(1).get_or_default())?,
            radius: input.field::<SchemaDouble>(2).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        <generated::improbable::Coordinates as TypeConversion>::to_type(&&input.center, &mut output.field::<SchemaObject>(1).add())?;
        output.field::<SchemaDouble>(2).add(input.radius);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_Query {
    pub constraint: generated::improbable::ComponentInterest_QueryConstraint,
    pub full_snapshot_result: Option<bool>,
    pub result_component_id: Vec<u32>,
    pub frequency: Option<f32>,
}
impl TypeConversion for ComponentInterest_Query {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            constraint: <generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::from_type(&input.field::<SchemaObject>(1).get_or_default())?,
            full_snapshot_result: if let Some(data) = input.field::<SchemaBool>(2).get() { Some(data) } else { None },
            result_component_id: { let size = input.field::<SchemaUint32>(3).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(input.field::<SchemaUint32>(3).index(i)); }; l },
            frequency: if let Some(data) = input.field::<SchemaFloat>(4).get() { Some(data) } else { None },
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        <generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::to_type(&&input.constraint, &mut output.field::<SchemaObject>(1).add())?;
        if let Some(data) = input.full_snapshot_result { output.field::<SchemaBool>(2).add(data); };
        output.field::<SchemaUint32>(3).add_list(&&input.result_component_id[..]);
        if let Some(data) = input.frequency { output.field::<SchemaFloat>(4).add(data); };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_QueryConstraint {
    pub sphere_constraint: Option<generated::improbable::ComponentInterest_SphereConstraint>,
    pub cylinder_constraint: Option<generated::improbable::ComponentInterest_CylinderConstraint>,
    pub box_constraint: Option<generated::improbable::ComponentInterest_BoxConstraint>,
    pub relative_sphere_constraint: Option<generated::improbable::ComponentInterest_RelativeSphereConstraint>,
    pub relative_cylinder_constraint: Option<generated::improbable::ComponentInterest_RelativeCylinderConstraint>,
    pub relative_box_constraint: Option<generated::improbable::ComponentInterest_RelativeBoxConstraint>,
    pub entity_id_constraint: Option<i64>,
    pub component_constraint: Option<u32>,
    pub and_constraint: Vec<generated::improbable::ComponentInterest_QueryConstraint>,
    pub or_constraint: Vec<generated::improbable::ComponentInterest_QueryConstraint>,
}
impl TypeConversion for ComponentInterest_QueryConstraint {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            sphere_constraint: if let Some(data) = input.field::<SchemaObject>(1).get() { Some(<generated::improbable::ComponentInterest_SphereConstraint as TypeConversion>::from_type(&data)?) } else { None },
            cylinder_constraint: if let Some(data) = input.field::<SchemaObject>(2).get() { Some(<generated::improbable::ComponentInterest_CylinderConstraint as TypeConversion>::from_type(&data)?) } else { None },
            box_constraint: if let Some(data) = input.field::<SchemaObject>(3).get() { Some(<generated::improbable::ComponentInterest_BoxConstraint as TypeConversion>::from_type(&data)?) } else { None },
            relative_sphere_constraint: if let Some(data) = input.field::<SchemaObject>(4).get() { Some(<generated::improbable::ComponentInterest_RelativeSphereConstraint as TypeConversion>::from_type(&data)?) } else { None },
            relative_cylinder_constraint: if let Some(data) = input.field::<SchemaObject>(5).get() { Some(<generated::improbable::ComponentInterest_RelativeCylinderConstraint as TypeConversion>::from_type(&data)?) } else { None },
            relative_box_constraint: if let Some(data) = input.field::<SchemaObject>(6).get() { Some(<generated::improbable::ComponentInterest_RelativeBoxConstraint as TypeConversion>::from_type(&data)?) } else { None },
            entity_id_constraint: if let Some(data) = input.field::<SchemaInt64>(7).get() { Some(data) } else { None },
            component_constraint: if let Some(data) = input.field::<SchemaUint32>(8).get() { Some(data) } else { None },
            and_constraint: { let size = input.field::<SchemaObject>(9).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(<generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::from_type(&input.field::<SchemaObject>(9).index(i))?); }; l },
            or_constraint: { let size = input.field::<SchemaObject>(10).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(<generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::from_type(&input.field::<SchemaObject>(10).index(i))?); }; l },
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        if let Some(ref data) = &input.sphere_constraint { <generated::improbable::ComponentInterest_SphereConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(1).add())?; };
        if let Some(ref data) = &input.cylinder_constraint { <generated::improbable::ComponentInterest_CylinderConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(2).add())?; };
        if let Some(ref data) = &input.box_constraint { <generated::improbable::ComponentInterest_BoxConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(3).add())?; };
        if let Some(ref data) = &input.relative_sphere_constraint { <generated::improbable::ComponentInterest_RelativeSphereConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(4).add())?; };
        if let Some(ref data) = &input.relative_cylinder_constraint { <generated::improbable::ComponentInterest_RelativeCylinderConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(5).add())?; };
        if let Some(ref data) = &input.relative_box_constraint { <generated::improbable::ComponentInterest_RelativeBoxConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(6).add())?; };
        if let Some(data) = input.entity_id_constraint { output.field::<SchemaInt64>(7).add(data); };
        if let Some(data) = input.component_constraint { output.field::<SchemaUint32>(8).add(data); };
        for element in (&input.and_constraint).iter() { <generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::to_type(&element, &mut output.field::<SchemaObject>(9).add())?; };
        for element in (&input.or_constraint).iter() { <generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::to_type(&element, &mut output.field::<SchemaObject>(10).add())?; };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_RelativeBoxConstraint {
    pub edge_length: generated::improbable::EdgeLength,
}
impl TypeConversion for ComponentInterest_RelativeBoxConstraint {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            edge_length: <generated::improbable::EdgeLength as TypeConversion>::from_type(&input.field::<SchemaObject>(1).get_or_default())?,
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        <generated::improbable::EdgeLength as TypeConversion>::to_type(&&input.edge_length, &mut output.field::<SchemaObject>(1).add())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_RelativeCylinderConstraint {
    pub radius: f64,
}
impl TypeConversion for ComponentInterest_RelativeCylinderConstraint {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            radius: input.field::<SchemaDouble>(1).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaDouble>(1).add(input.radius);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_RelativeSphereConstraint {
    pub radius: f64,
}
impl TypeConversion for ComponentInterest_RelativeSphereConstraint {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            radius: input.field::<SchemaDouble>(1).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaDouble>(1).add(input.radius);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_SphereConstraint {
    pub center: generated::improbable::Coordinates,
    pub radius: f64,
}
impl TypeConversion for ComponentInterest_SphereConstraint {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            center: <generated::improbable::Coordinates as TypeConversion>::from_type(&input.field::<SchemaObject>(1).get_or_default())?,
            radius: input.field::<SchemaDouble>(2).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        <generated::improbable::Coordinates as TypeConversion>::to_type(&&input.center, &mut output.field::<SchemaObject>(1).add())?;
        output.field::<SchemaDouble>(2).add(input.radius);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl TypeConversion for Coordinates {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            x: input.field::<SchemaDouble>(1).get_or_default(),
            y: input.field::<SchemaDouble>(2).get_or_default(),
            z: input.field::<SchemaDouble>(3).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaDouble>(1).add(input.x);
        output.field::<SchemaDouble>(2).add(input.y);
        output.field::<SchemaDouble>(3).add(input.z);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EdgeLength {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl TypeConversion for EdgeLength {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            x: input.field::<SchemaDouble>(1).get_or_default(),
            y: input.field::<SchemaDouble>(2).get_or_default(),
            z: input.field::<SchemaDouble>(3).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaDouble>(1).add(input.x);
        output.field::<SchemaDouble>(2).add(input.y);
        output.field::<SchemaDouble>(3).add(input.z);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WorkerAttributeSet {
    pub attribute: Vec<String>,
}
impl TypeConversion for WorkerAttributeSet {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            attribute: { let size = input.field::<SchemaString>(1).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(input.field::<SchemaString>(1).index(i)); }; l },
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaString>(1).add_list(&&input.attribute[..]);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WorkerRequirementSet {
    pub attribute_set: Vec<generated::improbable::WorkerAttributeSet>,
}
impl TypeConversion for WorkerRequirementSet {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            attribute_set: { let size = input.field::<SchemaObject>(1).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(<generated::improbable::WorkerAttributeSet as TypeConversion>::from_type(&input.field::<SchemaObject>(1).index(i))?); }; l },
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        for element in (&input.attribute_set).iter() { <generated::improbable::WorkerAttributeSet as TypeConversion>::to_type(&element, &mut output.field::<SchemaObject>(1).add())?; };
        Ok(())
    }
}

/* Components. */ 
#[derive(Debug, Clone)]
pub struct EntityAcl {
    pub read_acl: generated::improbable::WorkerRequirementSet,
    pub component_write_acl: BTreeMap<u32, generated::improbable::WorkerRequirementSet>,
}
impl TypeConversion for EntityAcl {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            read_acl: <generated::improbable::WorkerRequirementSet as TypeConversion>::from_type(&input.field::<SchemaObject>(1).get_or_default())?,
            component_write_acl: { let size = input.field::<SchemaObject>(2).count(); let mut m = BTreeMap::new(); for i in 0..size { let kv = input.field::<SchemaObject>(2).index(i); m.insert(kv.field::<SchemaUint32>(1).get_or_default(), <generated::improbable::WorkerRequirementSet as TypeConversion>::from_type(&kv.field::<SchemaObject>(2).get_or_default())?); }; m },
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        <generated::improbable::WorkerRequirementSet as TypeConversion>::to_type(&&input.read_acl, &mut output.field::<SchemaObject>(1).add())?;
        for (k, v) in &input.component_write_acl { let object = output.field::<SchemaObject>(2).add(); object.field::<SchemaUint32>(1).add(*k); <generated::improbable::WorkerRequirementSet as TypeConversion>::to_type(&v, &mut object.field::<SchemaObject>(2).add())?; };
        Ok(())
    }
}
impl ComponentData<EntityAcl> for EntityAcl {
    fn merge(&mut self, update: EntityAclUpdate) {
        if let Some(value) = update.read_acl { self.read_acl = value; }
        if let Some(value) = update.component_write_acl { self.component_write_acl = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EntityAclUpdate {
    pub read_acl: Option<generated::improbable::WorkerRequirementSet>,
    pub component_write_acl: Option<BTreeMap<u32, generated::improbable::WorkerRequirementSet>>,
}
impl TypeConversion for EntityAclUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            read_acl: None,
            component_write_acl: None,
        };
        let _field_read_acl = input.field::<SchemaObject>(1);
        if _field_read_acl.count() > 0 {
            let field = &_field_read_acl;
            output.read_acl = Some(<generated::improbable::WorkerRequirementSet as TypeConversion>::from_type(&field.get_or_default())?);
        }
        let _field_component_write_acl = input.field::<SchemaObject>(2);
        if _field_component_write_acl.count() > 0 {
            let field = &_field_component_write_acl;
            output.component_write_acl = Some({ let size = field.count(); let mut m = BTreeMap::new(); for i in 0..size { let kv = field.index(i); m.insert(kv.field::<SchemaUint32>(1).get_or_default(), <generated::improbable::WorkerRequirementSet as TypeConversion>::from_type(&kv.field::<SchemaObject>(2).get_or_default())?); }; m });
        }
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.read_acl {
            <generated::improbable::WorkerRequirementSet as TypeConversion>::to_type(&value, &mut output.field::<SchemaObject>(1).add())?;
        }
        if let Some(ref value) = input.component_write_acl {
            for (k, v) in value { let object = output.field::<SchemaObject>(2).add(); object.field::<SchemaUint32>(1).add(*k); <generated::improbable::WorkerRequirementSet as TypeConversion>::to_type(&v, &mut object.field::<SchemaObject>(2).add())?; };
        }
        Ok(())
    }
}
impl ComponentUpdate<EntityAcl> for EntityAclUpdate {
    fn merge(&mut self, update: EntityAclUpdate) {
        if update.read_acl.is_some() { self.read_acl = update.read_acl; }
        if update.component_write_acl.is_some() { self.component_write_acl = update.component_write_acl; }
    }
}

#[derive(Debug, Clone)]
pub enum EntityAclCommandRequest {
}

#[derive(Debug, Clone)]
pub enum EntityAclCommandResponse {
}

impl Component for EntityAcl {
    type Update = generated::improbable::EntityAclUpdate;
    type CommandRequest = generated::improbable::EntityAclCommandRequest;
    type CommandResponse = generated::improbable::EntityAclCommandResponse;

    const ID: ComponentId = 50;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::EntityAcl, String> {
        <generated::improbable::EntityAcl as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::EntityAclUpdate, String> {
        <generated::improbable::EntityAclUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::EntityAclCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component EntityAcl.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::EntityAclCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component EntityAcl.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::EntityAcl) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::EntityAcl as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::EntityAclUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::EntityAclUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::EntityAclCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::EntityAclCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::EntityAclCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::EntityAclCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<EntityAcl>());

#[derive(Debug, Clone)]
pub struct Interest {
    pub component_interest: BTreeMap<u32, generated::improbable::ComponentInterest>,
}
impl TypeConversion for Interest {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            component_interest: { let size = input.field::<SchemaObject>(1).count(); let mut m = BTreeMap::new(); for i in 0..size { let kv = input.field::<SchemaObject>(1).index(i); m.insert(kv.field::<SchemaUint32>(1).get_or_default(), <generated::improbable::ComponentInterest as TypeConversion>::from_type(&kv.field::<SchemaObject>(2).get_or_default())?); }; m },
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        for (k, v) in &input.component_interest { let object = output.field::<SchemaObject>(1).add(); object.field::<SchemaUint32>(1).add(*k); <generated::improbable::ComponentInterest as TypeConversion>::to_type(&v, &mut object.field::<SchemaObject>(2).add())?; };
        Ok(())
    }
}
impl ComponentData<Interest> for Interest {
    fn merge(&mut self, update: InterestUpdate) {
        if let Some(value) = update.component_interest { self.component_interest = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterestUpdate {
    pub component_interest: Option<BTreeMap<u32, generated::improbable::ComponentInterest>>,
}
impl TypeConversion for InterestUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            component_interest: None,
        };
        let _field_component_interest = input.field::<SchemaObject>(1);
        if _field_component_interest.count() > 0 {
            let field = &_field_component_interest;
            output.component_interest = Some({ let size = field.count(); let mut m = BTreeMap::new(); for i in 0..size { let kv = field.index(i); m.insert(kv.field::<SchemaUint32>(1).get_or_default(), <generated::improbable::ComponentInterest as TypeConversion>::from_type(&kv.field::<SchemaObject>(2).get_or_default())?); }; m });
        }
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.component_interest {
            for (k, v) in value { let object = output.field::<SchemaObject>(1).add(); object.field::<SchemaUint32>(1).add(*k); <generated::improbable::ComponentInterest as TypeConversion>::to_type(&v, &mut object.field::<SchemaObject>(2).add())?; };
        }
        Ok(())
    }
}
impl ComponentUpdate<Interest> for InterestUpdate {
    fn merge(&mut self, update: InterestUpdate) {
        if update.component_interest.is_some() { self.component_interest = update.component_interest; }
    }
}

#[derive(Debug, Clone)]
pub enum InterestCommandRequest {
}

#[derive(Debug, Clone)]
pub enum InterestCommandResponse {
}

impl Component for Interest {
    type Update = generated::improbable::InterestUpdate;
    type CommandRequest = generated::improbable::InterestCommandRequest;
    type CommandResponse = generated::improbable::InterestCommandResponse;

    const ID: ComponentId = 58;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::Interest, String> {
        <generated::improbable::Interest as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::InterestUpdate, String> {
        <generated::improbable::InterestUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::InterestCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Interest.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::InterestCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Interest.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::Interest) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::Interest as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::InterestUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::InterestUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::InterestCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::InterestCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::InterestCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::InterestCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Interest>());

#[derive(Debug, Clone)]
pub struct Metadata {
    pub entity_type: String,
}
impl TypeConversion for Metadata {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            entity_type: input.field::<SchemaString>(1).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaString>(1).add(&&input.entity_type);
        Ok(())
    }
}
impl ComponentData<Metadata> for Metadata {
    fn merge(&mut self, update: MetadataUpdate) {
        if let Some(value) = update.entity_type { self.entity_type = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MetadataUpdate {
    pub entity_type: Option<String>,
}
impl TypeConversion for MetadataUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            entity_type: None,
        };
        let _field_entity_type = input.field::<SchemaString>(1);
        if _field_entity_type.count() > 0 {
            let field = &_field_entity_type;
            output.entity_type = Some(field.get_or_default());
        }
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.entity_type {
            output.field::<SchemaString>(1).add(&value);
        }
        Ok(())
    }
}
impl ComponentUpdate<Metadata> for MetadataUpdate {
    fn merge(&mut self, update: MetadataUpdate) {
        if update.entity_type.is_some() { self.entity_type = update.entity_type; }
    }
}

#[derive(Debug, Clone)]
pub enum MetadataCommandRequest {
}

#[derive(Debug, Clone)]
pub enum MetadataCommandResponse {
}

impl Component for Metadata {
    type Update = generated::improbable::MetadataUpdate;
    type CommandRequest = generated::improbable::MetadataCommandRequest;
    type CommandResponse = generated::improbable::MetadataCommandResponse;

    const ID: ComponentId = 53;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::Metadata, String> {
        <generated::improbable::Metadata as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::MetadataUpdate, String> {
        <generated::improbable::MetadataUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::MetadataCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Metadata.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::MetadataCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Metadata.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::Metadata) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::Metadata as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::MetadataUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::MetadataUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::MetadataCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::MetadataCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::MetadataCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::MetadataCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Metadata>());

#[derive(Debug, Clone)]
pub struct Persistence {
}
impl TypeConversion for Persistence {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        Ok(())
    }
}
impl ComponentData<Persistence> for Persistence {
    fn merge(&mut self, update: PersistenceUpdate) {
    }
}

#[derive(Debug, Clone, Default)]
pub struct PersistenceUpdate {
}
impl TypeConversion for PersistenceUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
        };
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        Ok(())
    }
}
impl ComponentUpdate<Persistence> for PersistenceUpdate {
    fn merge(&mut self, update: PersistenceUpdate) {
    }
}

#[derive(Debug, Clone)]
pub enum PersistenceCommandRequest {
}

#[derive(Debug, Clone)]
pub enum PersistenceCommandResponse {
}

impl Component for Persistence {
    type Update = generated::improbable::PersistenceUpdate;
    type CommandRequest = generated::improbable::PersistenceCommandRequest;
    type CommandResponse = generated::improbable::PersistenceCommandResponse;

    const ID: ComponentId = 55;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::Persistence, String> {
        <generated::improbable::Persistence as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::PersistenceUpdate, String> {
        <generated::improbable::PersistenceUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::PersistenceCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Persistence.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::PersistenceCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Persistence.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::Persistence) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::Persistence as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::PersistenceUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::PersistenceUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::PersistenceCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::PersistenceCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::PersistenceCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::PersistenceCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Persistence>());

#[derive(Debug, Clone)]
pub struct Position {
    pub coords: generated::improbable::Coordinates,
}
impl TypeConversion for Position {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            coords: <generated::improbable::Coordinates as TypeConversion>::from_type(&input.field::<SchemaObject>(1).get_or_default())?,
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        <generated::improbable::Coordinates as TypeConversion>::to_type(&&input.coords, &mut output.field::<SchemaObject>(1).add())?;
        Ok(())
    }
}
impl ComponentData<Position> for Position {
    fn merge(&mut self, update: PositionUpdate) {
        if let Some(value) = update.coords { self.coords = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PositionUpdate {
    pub coords: Option<generated::improbable::Coordinates>,
}
impl TypeConversion for PositionUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            coords: None,
        };
        let _field_coords = input.field::<SchemaObject>(1);
        if _field_coords.count() > 0 {
            let field = &_field_coords;
            output.coords = Some(<generated::improbable::Coordinates as TypeConversion>::from_type(&field.get_or_default())?);
        }
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.coords {
            <generated::improbable::Coordinates as TypeConversion>::to_type(&value, &mut output.field::<SchemaObject>(1).add())?;
        }
        Ok(())
    }
}
impl ComponentUpdate<Position> for PositionUpdate {
    fn merge(&mut self, update: PositionUpdate) {
        if update.coords.is_some() { self.coords = update.coords; }
    }
}

#[derive(Debug, Clone)]
pub enum PositionCommandRequest {
}

#[derive(Debug, Clone)]
pub enum PositionCommandResponse {
}

impl Component for Position {
    type Update = generated::improbable::PositionUpdate;
    type CommandRequest = generated::improbable::PositionCommandRequest;
    type CommandResponse = generated::improbable::PositionCommandResponse;

    const ID: ComponentId = 54;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::Position, String> {
        <generated::improbable::Position as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::PositionUpdate, String> {
        <generated::improbable::PositionUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::PositionCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Position.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::PositionCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Position.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::Position) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::Position as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::PositionUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::PositionUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::PositionCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::PositionCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::PositionCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::PositionCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Position>());



pub mod restricted {
use spatialos_sdk::worker::internal::schema::*;
use spatialos_sdk::worker::component::*;
use std::collections::BTreeMap;

use super::super::super::generated as generated;

/* Enums. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Connection_ConnectionStatus {

    UNKNOWN,
    AWAITING_WORKER_CONNECTION,
    CONNECTED,
    DISCONNECTED,
}

impl From<u32> for Connection_ConnectionStatus {
    fn from(value: u32) -> Self {
        match value {

            0 => Connection_ConnectionStatus::UNKNOWN, 
            1 => Connection_ConnectionStatus::AWAITING_WORKER_CONNECTION, 
            2 => Connection_ConnectionStatus::CONNECTED, 
            3 => Connection_ConnectionStatus::DISCONNECTED, 
            _ => panic!(format!("Could not convert {} to enum Connection_ConnectionStatus.", value))
        }
    }
}

impl Connection_ConnectionStatus {
    pub(crate) fn as_u32(self) -> u32 {
        match self {
            
            Connection_ConnectionStatus::UNKNOWN => 0, 
            Connection_ConnectionStatus::AWAITING_WORKER_CONNECTION => 1, 
            Connection_ConnectionStatus::CONNECTED => 2, 
            Connection_ConnectionStatus::DISCONNECTED => 3, 
        }
    }
}

/* Types. */
#[derive(Debug, Clone)]
pub struct Connection {
    pub status: generated::improbable::restricted::Connection_ConnectionStatus,
    pub data_latency_ms: u32,
    pub connected_since_utc: u64,
}
impl TypeConversion for Connection {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            status: generated::improbable::restricted::Connection_ConnectionStatus::from(input.field::<SchemaEnum>(1).get_or_default()),
            data_latency_ms: input.field::<SchemaUint32>(2).get_or_default(),
            connected_since_utc: input.field::<SchemaUint64>(3).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaEnum>(1).add(input.status.as_u32());
        output.field::<SchemaUint32>(2).add(input.data_latency_ms);
        output.field::<SchemaUint64>(3).add(input.connected_since_utc);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DisconnectRequest {
}
impl TypeConversion for DisconnectRequest {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DisconnectResponse {
}
impl TypeConversion for DisconnectResponse {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PlayerIdentity {
    pub player_identifier: String,
    pub provider: String,
    pub metadata: Vec<u8>,
}
impl TypeConversion for PlayerIdentity {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            player_identifier: input.field::<SchemaString>(1).get_or_default(),
            provider: input.field::<SchemaString>(2).get_or_default(),
            metadata: input.field::<SchemaBytes>(3).get_or_default(),
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaString>(1).add(&&input.player_identifier);
        output.field::<SchemaString>(2).add(&&input.provider);
        output.field::<SchemaBytes>(3).add(&&input.metadata);
        Ok(())
    }
}

/* Components. */ 
#[derive(Debug, Clone)]
pub struct PlayerClient {
    pub player_identity: generated::improbable::restricted::PlayerIdentity,
}
impl TypeConversion for PlayerClient {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            player_identity: <generated::improbable::restricted::PlayerIdentity as TypeConversion>::from_type(&input.field::<SchemaObject>(1).get_or_default())?,
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        <generated::improbable::restricted::PlayerIdentity as TypeConversion>::to_type(&&input.player_identity, &mut output.field::<SchemaObject>(1).add())?;
        Ok(())
    }
}
impl ComponentData<PlayerClient> for PlayerClient {
    fn merge(&mut self, update: PlayerClientUpdate) {
        if let Some(value) = update.player_identity { self.player_identity = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerClientUpdate {
    pub player_identity: Option<generated::improbable::restricted::PlayerIdentity>,
}
impl TypeConversion for PlayerClientUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            player_identity: None,
        };
        let _field_player_identity = input.field::<SchemaObject>(1);
        if _field_player_identity.count() > 0 {
            let field = &_field_player_identity;
            output.player_identity = Some(<generated::improbable::restricted::PlayerIdentity as TypeConversion>::from_type(&field.get_or_default())?);
        }
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.player_identity {
            <generated::improbable::restricted::PlayerIdentity as TypeConversion>::to_type(&value, &mut output.field::<SchemaObject>(1).add())?;
        }
        Ok(())
    }
}
impl ComponentUpdate<PlayerClient> for PlayerClientUpdate {
    fn merge(&mut self, update: PlayerClientUpdate) {
        if update.player_identity.is_some() { self.player_identity = update.player_identity; }
    }
}

#[derive(Debug, Clone)]
pub enum PlayerClientCommandRequest {
}

#[derive(Debug, Clone)]
pub enum PlayerClientCommandResponse {
}

impl Component for PlayerClient {
    type Update = generated::improbable::restricted::PlayerClientUpdate;
    type CommandRequest = generated::improbable::restricted::PlayerClientCommandRequest;
    type CommandResponse = generated::improbable::restricted::PlayerClientCommandResponse;

    const ID: ComponentId = 61;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::restricted::PlayerClient, String> {
        <generated::improbable::restricted::PlayerClient as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::restricted::PlayerClientUpdate, String> {
        <generated::improbable::restricted::PlayerClientUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::restricted::PlayerClientCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component PlayerClient.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::restricted::PlayerClientCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component PlayerClient.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::restricted::PlayerClient) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::restricted::PlayerClient as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::restricted::PlayerClientUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::restricted::PlayerClientUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::restricted::PlayerClientCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::restricted::PlayerClientCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::restricted::PlayerClientCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::restricted::PlayerClientCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<PlayerClient>());

#[derive(Debug, Clone)]
pub struct System {
}
impl TypeConversion for System {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        Ok(())
    }
}
impl ComponentData<System> for System {
    fn merge(&mut self, update: SystemUpdate) {
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemUpdate {
}
impl TypeConversion for SystemUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
        };
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        Ok(())
    }
}
impl ComponentUpdate<System> for SystemUpdate {
    fn merge(&mut self, update: SystemUpdate) {
    }
}

#[derive(Debug, Clone)]
pub enum SystemCommandRequest {
}

#[derive(Debug, Clone)]
pub enum SystemCommandResponse {
}

impl Component for System {
    type Update = generated::improbable::restricted::SystemUpdate;
    type CommandRequest = generated::improbable::restricted::SystemCommandRequest;
    type CommandResponse = generated::improbable::restricted::SystemCommandResponse;

    const ID: ComponentId = 59;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::restricted::System, String> {
        <generated::improbable::restricted::System as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::restricted::SystemUpdate, String> {
        <generated::improbable::restricted::SystemUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::restricted::SystemCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component System.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::restricted::SystemCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component System.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::restricted::System) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::restricted::System as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::restricted::SystemUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::restricted::SystemUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::restricted::SystemCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::restricted::SystemCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::restricted::SystemCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::restricted::SystemCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<System>());

#[derive(Debug, Clone)]
pub struct Worker {
    pub worker_id: String,
    pub worker_type: String,
    pub connection: generated::improbable::restricted::Connection,
}
impl TypeConversion for Worker {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {
            worker_id: input.field::<SchemaString>(1).get_or_default(),
            worker_type: input.field::<SchemaString>(2).get_or_default(),
            connection: <generated::improbable::restricted::Connection as TypeConversion>::from_type(&input.field::<SchemaObject>(3).get_or_default())?,
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        output.field::<SchemaString>(1).add(&&input.worker_id);
        output.field::<SchemaString>(2).add(&&input.worker_type);
        <generated::improbable::restricted::Connection as TypeConversion>::to_type(&&input.connection, &mut output.field::<SchemaObject>(3).add())?;
        Ok(())
    }
}
impl ComponentData<Worker> for Worker {
    fn merge(&mut self, update: WorkerUpdate) {
        if let Some(value) = update.worker_id { self.worker_id = value; }
        if let Some(value) = update.worker_type { self.worker_type = value; }
        if let Some(value) = update.connection { self.connection = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkerUpdate {
    pub worker_id: Option<String>,
    pub worker_type: Option<String>,
    pub connection: Option<generated::improbable::restricted::Connection>,
}
impl TypeConversion for WorkerUpdate {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            worker_id: None,
            worker_type: None,
            connection: None,
        };
        let _field_worker_id = input.field::<SchemaString>(1);
        if _field_worker_id.count() > 0 {
            let field = &_field_worker_id;
            output.worker_id = Some(field.get_or_default());
        }
        let _field_worker_type = input.field::<SchemaString>(2);
        if _field_worker_type.count() > 0 {
            let field = &_field_worker_type;
            output.worker_type = Some(field.get_or_default());
        }
        let _field_connection = input.field::<SchemaObject>(3);
        if _field_connection.count() > 0 {
            let field = &_field_connection;
            output.connection = Some(<generated::improbable::restricted::Connection as TypeConversion>::from_type(&field.get_or_default())?);
        }
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.worker_id {
            output.field::<SchemaString>(1).add(&value);
        }
        if let Some(ref value) = input.worker_type {
            output.field::<SchemaString>(2).add(&value);
        }
        if let Some(ref value) = input.connection {
            <generated::improbable::restricted::Connection as TypeConversion>::to_type(&value, &mut output.field::<SchemaObject>(3).add())?;
        }
        Ok(())
    }
}
impl ComponentUpdate<Worker> for WorkerUpdate {
    fn merge(&mut self, update: WorkerUpdate) {
        if update.worker_id.is_some() { self.worker_id = update.worker_id; }
        if update.worker_type.is_some() { self.worker_type = update.worker_type; }
        if update.connection.is_some() { self.connection = update.connection; }
    }
}

#[derive(Debug, Clone)]
pub enum WorkerCommandRequest {
    Disconnect(generated::improbable::restricted::DisconnectRequest),
}

#[derive(Debug, Clone)]
pub enum WorkerCommandResponse {
    Disconnect(generated::improbable::restricted::DisconnectResponse),
}

impl Component for Worker {
    type Update = generated::improbable::restricted::WorkerUpdate;
    type CommandRequest = generated::improbable::restricted::WorkerCommandRequest;
    type CommandResponse = generated::improbable::restricted::WorkerCommandResponse;

    const ID: ComponentId = 60;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::restricted::Worker, String> {
        <generated::improbable::restricted::Worker as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::restricted::WorkerUpdate, String> {
        <generated::improbable::restricted::WorkerUpdate as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::restricted::WorkerCommandRequest, String> {
        match command_index {
            1 => {
                let result = <generated::improbable::restricted::DisconnectRequest as TypeConversion>::from_type(&request.object());
                result.and_then(|deserialized| Ok(WorkerCommandRequest::Disconnect(deserialized)))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Worker.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::restricted::WorkerCommandResponse, String> {
        match command_index {
            1 => {
                let result = <generated::improbable::restricted::DisconnectResponse as TypeConversion>::from_type(&response.object());
                result.and_then(|deserialized| Ok(WorkerCommandResponse::Disconnect(deserialized)))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Worker.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::restricted::Worker) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::restricted::Worker as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::restricted::WorkerUpdate) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::restricted::WorkerUpdate as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::restricted::WorkerCommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            WorkerCommandRequest::Disconnect(ref data) => {
                <generated::improbable::restricted::DisconnectRequest as TypeConversion>::to_type(data, &mut serialized_request.object_mut())?;
            },
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::restricted::WorkerCommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            WorkerCommandResponse::Disconnect(ref data) => {
                <generated::improbable::restricted::DisconnectResponse as TypeConversion>::to_type(data, &mut serialized_response.object_mut())?;
            },
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::restricted::WorkerCommandRequest) -> u32 {
        match request {
            WorkerCommandRequest::Disconnect(_) => 1,
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::restricted::WorkerCommandResponse) -> u32 {
        match response {
            WorkerCommandResponse::Disconnect(_) => 1,
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Worker>());


}
}
