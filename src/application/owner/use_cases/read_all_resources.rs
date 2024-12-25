use async_trait::async_trait;

use crate::{
    application::{
        owner::resource::repository::ResourceRepository,
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
    resource_repository: &'b B,
}

impl<'b, B> ReadAllResources<'b, B>
where
    B: ResourceRepository,
{
    pub fn new(resource_repository: &'b B) -> Self {
        Self {
            resource_repository,
        }
    }
}

#[async_trait(?Send)]
impl<'b, B> UseCase for ReadAllResources<'b, B>
where
    B: ResourceRepository,
{
    type Request = Request<'b>;
    type Response = Response;

    async fn execute(&self, request: Request<'b>) -> Result {
        let resources = self
            .resource_repository
            .find_by_owner(request.owner_id)
            .await?;
        Ok(Response { resources })
    }
}

#[cfg(test)]
mod owner_use_case_read_all_resources_tests {}
