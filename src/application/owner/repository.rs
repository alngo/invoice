use crate::application::shared::error::ApplicationError;
use crate::domain::owner::{Owner, OwnerEvent, OwnerId, Resource, ResourceId};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait OwnerRepository {
    async fn next_owner_id(&self) -> Result<OwnerId, ApplicationError>;
    async fn next_resource_id(&self) -> Result<ResourceId, ApplicationError>;
    async fn find_owner_by_id(&self, owner_id: &OwnerId) -> Result<Owner, ApplicationError>;
    async fn find_resource_by_id(
        &self,
        owner_id: &OwnerId,
        resource_id: &ResourceId,
    ) -> Result<Resource, ApplicationError>;
    async fn store(
        &self,
        owner_id: &OwnerId,
        events: Vec<OwnerEvent>,
    ) -> Result<(), ApplicationError>;
}
