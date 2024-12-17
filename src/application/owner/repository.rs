use crate::application::shared::error::ApplicationError;
use crate::domain::owner::{Owner, OwnerEvent, OwnerId};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait OwnerRepository {
    async fn next_id(&self) -> Result<OwnerId, ApplicationError>;
    async fn find_by_id(&self, owner_id: OwnerId) -> Result<Owner, ApplicationError>;
    async fn store(&self, events: Vec<OwnerEvent>) -> Result<(), ApplicationError>;
}
