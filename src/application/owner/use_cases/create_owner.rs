use crate::{
    application::{
        owner::repository::OwnerRepository,
        shared::{error::ApplicationError, use_case::UseCase},
    },
    domain::{
        owner::{Owner, OwnerCommand},
        shared::Aggregate,
    },
};
use async_trait::async_trait;

pub struct Request<'a> {
    pub name: &'a str,
}

pub struct Response {
    pub name: String,
}

pub type Result = core::result::Result<Response, ApplicationError>;

pub struct CreateOwner<'a, A> {
    owner_repository: &'a A,
}

impl<'a, A> CreateOwner<'a, A>
where
    A: OwnerRepository,
{
    pub fn new(owner_repository: &'a A) -> Self {
        Self { owner_repository }
    }
}

#[async_trait(?Send)]
impl<'a, A> UseCase for CreateOwner<'a, A>
where
    A: OwnerRepository,
{
    type Request = Request<'a>;
    type Response = Response;

    async fn execute(&self, request: Request<'a>) -> Result {
        let id = self.owner_repository.next_owner_id().await?;
        let command = OwnerCommand::CreateOwner {
            id,
            name: request.name.to_string(),
        };
        let owner = Owner::default();
        let events = owner.handle(command)?;
        self.owner_repository.store(&id, events).await?;
        Ok(Response {
            name: request.name.to_string(),
        })
    }
}

#[cfg(test)]
mod owner_use_case_create_owner_tests {
    use super::*;
    use crate::{
        application::owner::repository::MockOwnerRepository,
        domain::owner::{OwnerEvent, OwnerId},
    };

    #[tokio::test]
    async fn test_create_owner() {
        let mut owner_repository = MockOwnerRepository::new();
        owner_repository.expect_store().returning(|_, events| {
            assert_eq!(events.len(), 1);
            assert!(matches!(&events[0], OwnerEvent::OwnerCreated { .. }));
            Ok(())
        });
        owner_repository
            .expect_next_owner_id()
            .returning(|| Ok(OwnerId::default()));

        let use_case = CreateOwner::new(&owner_repository);
        let request = Request { name: "owner" };
        let response = use_case.execute(request).await.unwrap();
        assert_eq!(response.name, "owner");
    }

    #[tokio::test]
    async fn test_create_owner_error() {
        let mut owner_repository = MockOwnerRepository::new();
        owner_repository
            .expect_next_owner_id()
            .returning(|| Ok(OwnerId::default()));
        owner_repository.expect_store().returning(|_, _| {
            Err(ApplicationError {
                message: String::from("error"),
            })
        });

        let use_case = CreateOwner::new(&owner_repository);
        let request = Request { name: "owner" };
        let response = use_case.execute(request).await;
        assert!(response.is_err());
    }
}
