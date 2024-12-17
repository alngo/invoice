use crate::application::shared::error::ApplicationError;
use crate::domain::owner::{Owner, OwnerEvent, OwnerId};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait OwnerRepository {
    async fn add(&self, owner: Owner) -> Result<(), ApplicationError>;
    async fn update(&self, owner: Owner) -> Result<(), ApplicationError>;
    async fn remove(&self, owner_id: OwnerId) -> Result<(), ApplicationError>;
    async fn find(&self, owner_id: OwnerId) -> Result<(), ApplicationError>;
    async fn find_by_name(&self, name: String) -> Result<(), ApplicationError>;
    async fn find_all(&self) -> Result<(), ApplicationError>;
    async fn store(&self, events: Vec<OwnerEvent>) -> Result<(), ApplicationError>;
}
