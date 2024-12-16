mod command;
mod email;
mod event;
mod repository;
mod resource;

use super::shared::{aggregate::Aggregate, entity::Entity, error::DomainError};
pub use command::OwnerCommand;
pub use email::Email;
pub use event::OwnerEvent;
pub use resource::*;

pub type OwnerId = uuid::Uuid;

#[derive(Debug, Default)]
pub struct Owner {
    name: String,
    email: Email,
    resources: Vec<Resource>,
}

impl Entity for Owner {}

impl Aggregate for Owner {
    type Command = OwnerCommand;
    type Event = OwnerEvent;

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, DomainError> {
        match command {
            OwnerCommand::UpdateName { name } => Ok(vec![OwnerEvent::NameUpdated { name }]),
            OwnerCommand::UpdateEmail { email } => Ok(vec![OwnerEvent::EmailUpdated { email }]),
            OwnerCommand::AddResource {
                name,
                description,
                price,
            } => {
                let resource_id = ResourceId::now_v7();
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
            } => Ok(vec![OwnerEvent::ResourceUpdated {
                resource_id,
                name,
                description,
                price,
            }]),
            OwnerCommand::RemoveResource { resource_id } => {
                Ok(vec![OwnerEvent::ResourceRemoved { resource_id }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
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
            OwnerEvent::NameUpdated { name } => {
                self.name = name;
            }
            OwnerEvent::EmailUpdated { email } => {
                self.email = email;
            }
        }
    }
}
