use async_trait::async_trait;

use crate::{
    application::{
        owner::repository::OwnerRepository,
        shared::{error::ApplicationError, use_case::UseCase},
    },
    domain::{
        owner::{OwnerCommand, OwnerId, ResourceId},
        shared::Aggregate,
    },
};

pub struct Request<'a> {
    pub owner_id: &'a OwnerId,
    pub resource_id: &'a ResourceId,
}

pub struct Response {
    pub resource_id: ResourceId,
}

pub type Result = core::result::Result<Response, ApplicationError>;

pub struct RemoveResource<'a, A> {
    owner_repository: &'a A,
}

impl<'a, A> RemoveResource<'a, A>
where
    A: OwnerRepository,
{
    pub fn new(owner_repository: &'a A) -> Self {
        Self { owner_repository }
    }
}

#[async_trait(?Send)]
impl<'a, A> UseCase for RemoveResource<'a, A>
where
    A: OwnerRepository,
{
    type Request = Request<'a>;
    type Response = Response;

    async fn execute(&self, request: Request<'a>) -> Result {
        let owner = self
            .owner_repository
            .find_owner_by_id(request.owner_id)
            .await?;
        let command = OwnerCommand::RemoveResource {
            resource_id: *request.resource_id,
        };
        let events = owner.handle(command)?;
        self.owner_repository.store(owner.id(), events).await?;
        Ok(Response {
            resource_id: *request.resource_id,
        })
    }
}

#[cfg(test)]
mod owner_use_case_remove_resource_tests {}
