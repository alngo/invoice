use crate::{
    application::shared::error::ApplicationError,
    domain::owner::{OwnerId, Resource, ResourceId},
};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait ResourceRepository {
    async fn next_id(&self) -> Result<ResourceId, ApplicationError>;
    async fn find_by_owner(&self, owner_id: &OwnerId) -> Result<Vec<Resource>, ApplicationError>;
}
