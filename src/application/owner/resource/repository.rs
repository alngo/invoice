use crate::{
    application::shared::error::ApplicationError,
    domain::owner::{Resource, ResourceId},
};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait ResourceRepository {
    async fn add(&self, resource: Resource) -> Result<(), ApplicationError>;
    async fn update(&self, resource: Resource) -> Result<(), ApplicationError>;
    async fn remove(&self, resource_id: ResourceId) -> Result<(), ApplicationError>;
    async fn find(&self, resource_id: ResourceId) -> Result<Resource, ApplicationError>;
    async fn find_all(&self) -> Result<Vec<Resource>, ApplicationError>;
}
