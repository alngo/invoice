use crate::{
    adapter::shared::Present,
    application::{
        owner::{
            add_resource::{self, AddResource},
            create_owner::{self, CreateOwner},
            read_all_resources::{self, ReadAllResources},
            remove_resource::{self, RemoveResource},
            OwnerRepository,
        },
        UseCase,
    },
    domain::owner::{OwnerId, Price, ResourceId},
};

pub struct Controller<'a, 'c, A, C> {
    repositories: &'a A,
    presenter: &'c C,
}

impl<'a, 'c, A, C> Controller<'a, 'c, A, C>
where
    A: OwnerRepository,
    C: Present<create_owner::Result>
        + Present<add_resource::Result>
        + Present<remove_resource::Result>
        + Present<read_all_resources::Result>,
{
    pub const fn new(repositories: &'a A, presenter: &'c C) -> Self {
        Self {
            repositories,
            presenter,
        }
    }

    pub async fn create_owner(
        &self,
        name: &str,
    ) -> <C as Present<create_owner::Result>>::ViewModel {
        let request = create_owner::Request { name };
        let use_case = CreateOwner::new(self.repositories);
        let result = use_case.execute(request).await;
        self.presenter.present(result)
    }

    pub async fn add_resource(
        &self,
        owner_id: &OwnerId,
        name: &str,
        description: &str,
        price: Price,
    ) -> <C as Present<add_resource::Result>>::ViewModel {
        let use_case = AddResource::new(self.repositories);
        let request = add_resource::Request {
            owner_id,
            name,
            description,
            price,
        };
        let result = use_case.execute(request).await;
        self.presenter.present(result)
    }

    pub async fn remove_resource(
        &self,
        owner_id: &OwnerId,
        resource_id: &ResourceId,
    ) -> <C as Present<remove_resource::Result>>::ViewModel {
        let use_case = RemoveResource::new(self.repositories);
        let request = remove_resource::Request {
            owner_id,
            resource_id,
        };
        let result = use_case.execute(request).await;
        self.presenter.present(result)
    }

    pub async fn read_all_resources(
        &self,
        owner_id: &OwnerId,
    ) -> <C as Present<read_all_resources::Result>>::ViewModel {
        let use_case = ReadAllResources::new(self.repositories);
        let request = read_all_resources::Request { owner_id };
        let result = use_case.execute(request).await;
        self.presenter.present(result)
    }
}

#[cfg(test)]
mod owner_controller_tests {}
