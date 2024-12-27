use async_trait::async_trait;

use crate::{
    application::{
        owner::OwnerRepository,
        shared::{error::ApplicationError, use_case::UseCase},
    },
    domain::owner::{OwnerId, Resource},
};

pub struct Request<'b> {
    pub owner_id: &'b OwnerId,
}

pub struct Response {
    pub resources: Vec<Resource>,
}

pub type Result = core::result::Result<Response, ApplicationError>;

pub struct ReadAllResources<'b, B> {
    owner_repository: &'b B,
}

impl<'b, B> ReadAllResources<'b, B>
where
    B: OwnerRepository,
{
    pub fn new(owner_repository: &'b B) -> Self {
        Self { owner_repository }
    }
}

#[async_trait(?Send)]
impl<'b, B> UseCase for ReadAllResources<'b, B>
where
    B: OwnerRepository,
{
    type Request = Request<'b>;
    type Response = Response;

    async fn execute(&self, request: Request<'b>) -> Result {
        let owner = self
            .owner_repository
            .find_owner_by_id(request.owner_id)
            .await?;
        let resources = owner.resources();
        Ok(Response {
            resources: resources.to_vec(),
        })
    }
}

#[cfg(test)]
mod owner_use_case_read_all_resources_tests {}
