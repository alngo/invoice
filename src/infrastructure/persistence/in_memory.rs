use std::{collections::HashMap, sync::RwLock};

use async_trait::async_trait;

use crate::{
    application::{owner::OwnerRepository, ApplicationError},
    domain::{
        owner::{Owner, OwnerEvent, OwnerId, Resource, ResourceId},
        shared::Aggregate,
    },
};

pub struct InMemory {
    events: RwLock<HashMap<OwnerId, Vec<OwnerEvent>>>,
}

#[async_trait(?Send)]
impl OwnerRepository for InMemory {
    async fn next_owner_id(&self) -> Result<OwnerId, ApplicationError> {
        Ok(OwnerId::now_v7())
    }

    async fn next_resource_id(&self) -> Result<ResourceId, ApplicationError> {
        Ok(ResourceId::now_v7())
    }

    async fn find_owner_by_id(&self, owner_id: &OwnerId) -> Result<Owner, ApplicationError> {
        let mut owner = Owner::default();
        if let Some(events) = self.events.read().unwrap().get(&owner_id) {
            for event in events {
                owner.apply(event.clone());
            }
        }
        Ok(owner)
    }

    async fn find_resource_by_id(
        &self,
        owner_id: &OwnerId,
        resource_id: &ResourceId,
    ) -> Result<Resource, ApplicationError> {
        let owner = self.find_owner_by_id(owner_id).await?;
        let resource = owner
            .resources()
            .iter()
            .find(|r| r.id() == resource_id)
            .unwrap()
            .clone();
        Ok(resource)
    }

    async fn store(
        &self,
        owner_id: &OwnerId,
        new_events: Vec<OwnerEvent>,
    ) -> Result<(), ApplicationError> {
        if let Some(events) = self.events.write().unwrap().get_mut(owner_id) {
            events.extend(new_events);
        } else {
            self.events
                .write()
                .unwrap()
                .insert(owner_id.clone(), new_events);
        }
        Ok(())
    }
}
