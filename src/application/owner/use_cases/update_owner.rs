use crate::{
    application::{
        owner::repository::OwnerRepository,
        shared::{error::ApplicationError, use_case::UseCase},
    },
    domain::{
        owner::{OwnerCommand, OwnerId},
        shared::Aggregate,
    },
};
use async_trait::async_trait;

pub struct Request<'a> {
    pub id: &'a OwnerId,
    pub name: &'a str,
}

pub struct Response {
    pub id: OwnerId,
}

pub type Result = core::result::Result<Response, ApplicationError>;

pub struct UpdateOwner<'a, A> {
    owner_repository: &'a A,
}

impl<'a, A> UpdateOwner<'a, A>
where
    A: OwnerRepository,
{
    pub fn new(owner_repository: &'a A) -> Self {
        Self { owner_repository }
    }
}

#[async_trait(?Send)]
impl<'a, A> UseCase for UpdateOwner<'a, A>
where
    A: OwnerRepository,
{
    type Request = Request<'a>;
    type Response = Response;

    async fn execute(&self, request: Request<'a>) -> Result {
        let command = OwnerCommand::UpdateOwner {
            id: *request.id,
            name: request.name.to_string(),
        };
        let owner = self.owner_repository.find_by_id(*request.id).await?;
        let events = owner.handle(command)?;
        self.owner_repository.store(events).await?;
        Ok(Response { id: *request.id })
    }
}

#[cfg(test)]
mod owner_use_case_update_owner_tests {
    use super::*;
    use crate::{
        application::owner::repository::MockOwnerRepository,
        domain::owner::{Owner, OwnerEvent},
    };

    #[tokio::test]
    async fn test_update_owner() {
        let mut owner_repository = MockOwnerRepository::new();
        owner_repository
            .expect_find_by_id()
            .returning(|_| Ok(Owner::default()));
        owner_repository.expect_store().returning(|events| {
            assert_eq!(events.len(), 1);
            assert!(matches!(&events[0], OwnerEvent::OwnerUpdated { .. }));
            Ok(())
        });

        let use_case = UpdateOwner::new(&owner_repository);
        let request = Request {
            id: &OwnerId::default(),
            name: "owner",
        };
        let response = use_case.execute(request).await.unwrap();
        assert_eq!(response.id, OwnerId::default());
    }

    #[tokio::test]
    async fn test_update_owner_error_owner_not_found() {
        let mut owner_repository = MockOwnerRepository::new();
        owner_repository.expect_find_by_id().returning(|_| {
            Err(ApplicationError {
                message: String::from("error"),
            })
        });

        let use_case = UpdateOwner::new(&owner_repository);
        let request = Request {
            id: &OwnerId::default(),
            name: "owner",
        };
        let response = use_case.execute(request).await;
        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_update_owner_error_from_storage() {
        let mut owner_repository = MockOwnerRepository::new();
        owner_repository
            .expect_find_by_id()
            .returning(|_| Ok(Owner::default()));
        owner_repository.expect_store().returning(|_| {
            Err(ApplicationError {
                message: String::from("error"),
            })
        });

        let use_case = UpdateOwner::new(&owner_repository);
        let request = Request {
            id: &OwnerId::default(),
            name: "owner",
        };
        let response = use_case.execute(request).await;
        assert!(response.is_err());
    }
}
