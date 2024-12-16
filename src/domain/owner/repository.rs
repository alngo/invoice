use crate::domain::shared::error::DomainError;
use async_trait::async_trait;

use super::{Owner, OwnerId};

#[async_trait(?Send)]
pub trait OwnerRepository {
    async fn add(&self, owner: Owner) -> Result<(), DomainError>;
    async fn update(&self, owner: Owner) -> Result<(), DomainError>;
    async fn remove(&self, owner_id: OwnerId) -> Result<(), DomainError>;
    async fn find(&self, owner_id: OwnerId) -> Result<(), DomainError>;
    async fn find_all(&self) -> Result<(), DomainError>;
}
