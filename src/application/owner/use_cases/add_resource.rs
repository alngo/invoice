use async_trait::async_trait;

use crate::{
    application::{
        owner::{repository::OwnerRepository, resource::repository::ResourceRepository},
        shared::{error::ApplicationError, use_case::UseCase},
    },
    domain::{
        owner::{OwnerCommand, OwnerId, Price, ResourceId},
        shared::Aggregate,
    },
};

pub struct Request<'a> {
    pub owner_id: &'a OwnerId,
    pub name: &'a str,
    pub description: &'a str,
    pub price: Price,
}

pub struct Response {
    pub resource_id: ResourceId,
}

pub type Result = core::result::Result<Response, ApplicationError>;

pub struct AddResource<'a, 'b, A, B> {
    owner_repository: &'a A,
    resource_repository: &'b B,
}

impl<'a, 'b, A, B> AddResource<'a, 'b, A, B>
where
    A: OwnerRepository,
    B: ResourceRepository,
{
    pub fn new(owner_repository: &'a A, resource_repository: &'b B) -> Self {
        Self {
            owner_repository,
            resource_repository,
        }
    }
}

#[async_trait(?Send)]
impl<'a, 'b, A, B> UseCase for AddResource<'a, 'b, A, B>
where
    A: OwnerRepository,
    B: ResourceRepository,
{
    type Request = Request<'a>;
    type Response = Response;

    async fn execute(&self, request: Request<'a>) -> Result {
        let owner = self.owner_repository.find_by_id(*request.owner_id).await?;
        let id = self.resource_repository.next_id().await?;
        let command = OwnerCommand::AddResource {
            id,
            name: request.name.to_string(),
            description: request.description.to_string(),
            price: request.price,
        };
        let events = owner.handle(command)?;
        let id = self.resource_repository.next_id().await?;
        self.owner_repository.store(events).await?;
        Ok(Response { resource_id: id })
    }
}

#[cfg(test)]
mod owner_use_case_add_resource_tests {
    use super::*;
    use crate::{
        application::owner::{
            repository::MockOwnerRepository, resource::repository::MockResourceRepository,
        },
        domain::owner::Owner,
    };

    #[tokio::test]
    async fn add_resource() {
        let mut owner_repository = MockOwnerRepository::new();
        owner_repository
            .expect_find_by_id()
            .returning(|_| Ok(Owner::default()));
        owner_repository.expect_store().returning(|_| Ok(()));
        let mut resource_repository = MockResourceRepository::new();
        resource_repository
            .expect_next_id()
            .returning(|| Ok(ResourceId::default()));
        let use_case = AddResource::new(&owner_repository, &resource_repository);

        let request = Request {
            owner_id: &OwnerId::default(),
            name: "resource_name",
            description: "resource_description",
            price: Price::from_f32(100.00).unwrap(),
        };

        let response = use_case.execute(request).await.unwrap();

        assert_eq!(response.resource_id, ResourceId::default());
    }
}
