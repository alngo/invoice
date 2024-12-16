use super::{Resource, ResourceId};
use crate::domain::shared::error::DomainError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait ResourceRepository {
    async fn add(&self, resource: Resource) -> Result<(), DomainError>;
    async fn update(&self, resource: Resource) -> Result<(), DomainError>;
    async fn remove(&self, resource_id: ResourceId) -> Result<(), DomainError>;
    async fn find(&self, resource_id: ResourceId) -> Result<Resource, DomainError>;
    async fn find_all(&self) -> Result<Vec<Resource>, DomainError>;
}
