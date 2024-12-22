mod command;
mod event;
mod resource;
mod rules;

use super::shared::{Aggregate, DomainError, Entity};
pub use command::OwnerCommand;
pub use event::OwnerEvent;
pub use resource::*;
use rules::{
    OwnerIdMustMatch, OwnerMustOwnTheResource, ResourceIdMustBeUnique, ResourceNameMustBeUnique,
};

pub type OwnerId = uuid::Uuid;

#[derive(Debug, Default)]
pub struct Owner {
    id: OwnerId,
    name: String,
    resources: Vec<Resource>,
}
impl Owner {
    pub fn id(&self) -> &OwnerId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Entity for Owner {}

impl Aggregate for Owner {
    type Command = OwnerCommand;
    type Event = OwnerEvent;

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, DomainError> {
        match command {
            OwnerCommand::CreateOwner { id, name } => Ok(vec![OwnerEvent::OwnerCreated {
                id,
                name: name.to_string(),
            }]),
            OwnerCommand::UpdateOwner { id, name } => {
                Self::check_rule(OwnerIdMustMatch::new(&self.id, &id))?;
                Ok(vec![OwnerEvent::OwnerUpdated { name }])
            }
            OwnerCommand::AddResource {
                id,
                name,
                description,
                price,
            } => {
                let resource_id = ResourceId::now_v7();
                Self::check_rule(ResourceNameMustBeUnique::new(&name, &self.resources))?;
                Self::check_rule(ResourceIdMustBeUnique::new(&id, &self.resources))?;
                Ok(vec![OwnerEvent::ResourceAdded {
                    resource_id,
                    name,
                    description,
                    price,
                }])
            }
            OwnerCommand::UpdateResource {
                resource_id,
                name,
                description,
                price,
            } => {
                Self::check_rule(OwnerMustOwnTheResource::new(&resource_id, &self.resources))?;
                Self::check_rule(ResourceNameMustBeUnique::new(&name, &self.resources))?;
                Ok(vec![OwnerEvent::ResourceUpdated {
                    resource_id,
                    name,
                    description,
                    price,
                }])
            }
            OwnerCommand::RemoveResource { resource_id } => {
                Self::check_rule(OwnerMustOwnTheResource::new(&resource_id, &self.resources))?;
                Ok(vec![OwnerEvent::ResourceRemoved { resource_id }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            OwnerEvent::OwnerCreated { id, name } => {
                self.id = id;
                self.name = name;
            }
            OwnerEvent::OwnerUpdated { name } => {
                self.name = name;
            }
            OwnerEvent::ResourceAdded {
                resource_id,
                name,
                description,
                price,
            } => {
                let resource = Resource::new(resource_id, name, description, price);
                self.resources.push(resource);
            }
            OwnerEvent::ResourceUpdated {
                resource_id,
                name,
                description,
                price,
            } => {
                let resource = Resource::new(resource_id, name, description, price);
                self.resources
                    .retain(|resource| resource.id() != &resource_id);
                self.resources.push(resource);
            }
            OwnerEvent::ResourceRemoved { resource_id } => {
                self.resources
                    .retain(|resource| resource.id() != &resource_id);
            }
        }
    }
}
