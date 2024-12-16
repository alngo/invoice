mod command;
mod email;
mod event;
mod repository;
mod resource;
mod rules;

use super::shared::{aggregate::Aggregate, entity::Entity, error::DomainError};
pub use command::OwnerCommand;
pub use email::Email;
pub use event::OwnerEvent;
pub use resource::*;
use rules::{OwnerIdMustMatch, OwnerMustOwnTheResource, ResourceNameMustBeUnique};

pub type OwnerId = uuid::Uuid;

#[derive(Debug, Default)]
pub struct Owner {
    id: OwnerId,
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
            OwnerCommand::CreateOwner { name, email } => {
                let id = OwnerId::now_v7();
                Ok(vec![OwnerEvent::OwnerCreated { id, name, email }])
            }
            OwnerCommand::UpdateOwner { id, name, email } => {
                Self::check_rule(OwnerIdMustMatch::new(&self.id, &id))?;
                Ok(vec![OwnerEvent::OwnerUpdated { name, email }])
            }
            OwnerCommand::AddResource {
                name,
                description,
                price,
            } => {
                let resource_id = ResourceId::now_v7();
                Self::check_rule(ResourceNameMustBeUnique::new(&name, &self.resources))?;
                Ok(vec![OwnerEvent::ResourceAdded { resource_id, name, description, price }])
            }
            OwnerCommand::UpdateResource {
                resource_id,
                name,
                description,
                price,
            } => {
                Self::check_rule(OwnerMustOwnTheResource::new(&resource_id, &self.resources))?;
                Self::check_rule(ResourceNameMustBeUnique::new(&name, &self.resources))?;
                Ok(vec![OwnerEvent::ResourceUpdated { resource_id, name, description, price }])
            }
            OwnerCommand::RemoveResource { resource_id } => {
                Self::check_rule(OwnerMustOwnTheResource::new(&resource_id, &self.resources))?;
                Ok(vec![OwnerEvent::ResourceRemoved { resource_id }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            OwnerEvent::ResourceAdded { resource_id, name, description, price } => {
                let resource = Resource::new(resource_id, name, description, price);
                self.resources.push(resource);
            }
            OwnerEvent::ResourceUpdated { resource_id, name, description, price } => {
                let resource = Resource::new(resource_id, name, description, price);
                self.resources.retain(|resource| resource.id() != &resource_id);
                self.resources.push(resource);
            }
            OwnerEvent::ResourceRemoved { resource_id } => {
                self.resources.retain(|resource| resource.id() != &resource_id);
            }
            OwnerEvent::OwnerUpdated { name, email } => {
                self.name = name;
                self.email = email;
            }
            OwnerEvent::OwnerCreated { id, name, email } => {
                self.id = id;
                self.name = name;
                self.email = email;
            }
        }
    }
}
