use crate::{adapter::shared::Present, application::{owner::{add_resource::{self, AddResource}, create_owner::{self, CreateOwner}, remove_resource::{self, RemoveResource}, OwnerRepository, ResourceRepository}, UseCase}, domain::owner::{OwnerId, Price, ResourceId}};

pub struct Controller<'a, 'b, 'c, A, B, C> {
    owner_repository: &'a A,
    resource_repository: &'b B,
    presenter: &'c C,
}

impl<'a, 'b, 'c, A, B, C> Controller<'a, 'b, 'c, A, B, C>
where
    A: OwnerRepository,
    B: ResourceRepository,
    C: Present<create_owner::Result> + Present<add_resource::Result> + Present<remove_resource::Result>
{
    pub const fn new(
    owner_repository: &'a A,
    resource_repository: &'b B,
    presenter: &'c C,
) -> Self {
        Self {owner_repository, resource_repository, presenter }
    }

    pub async fn create_owner(&self, name: &str) -> <C as Present<create_owner::Result>>::ViewModel {
        let request = create_owner::Request {
            name
        };
        let use_case = CreateOwner::new(self.owner_repository);
        let result = use_case.execute(request).await;
        self.presenter.present(result)
    }

    pub async fn add_resource(&self, 
        owner_id: &OwnerId, 
        name: &str, 
        description: &str, 
        price: f32
        ) -> <C as Present<add_resource::Result>>::ViewModel {
        let use_case = AddResource::new(self.owner_repository, self.resource_repository);
        let request = add_resource::Request {
            owner_id,
            name,
            description,
            price: Price::from_f32(price).unwrap()
        };
        let result = use_case.execute(request).await;
        self.presenter.present(result)
    }

    pub async fn remove_resource(&self, 
        owner_id: &OwnerId, 
        resource_id: &ResourceId,
        ) -> <C as Present<remove_resource::Result>>::ViewModel {
        let use_case = RemoveResource::new(self.owner_repository, self.resource_repository);
        let request = remove_resource::Request {
            owner_id,
            resource_id,
        };
        let result = use_case.execute(request).await;
        self.presenter.present(result)
    }
}

#[cfg(test)]
mod owner_controller_tests {
}
