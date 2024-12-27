use std::sync::Arc;

use crate::{
    application::owner::{
        add_resource, create_owner, read_all_resources, remove_resource, OwnerRepository,
    },
    domain::owner::OwnerId,
};

use super::{owner::Controller, shared::Present};

pub struct Api<D, P> {
    database: Arc<D>,
    presenter: P,
}

impl<D, P> Clone for Api<D, P>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            database: Arc::clone(&self.database),
            presenter: self.presenter.clone(),
        }
    }
}

impl<D, P> Api<D, P>
where
    D: OwnerRepository,
    P: Present<create_owner::Result>
        + Present<add_resource::Result>
        + Present<remove_resource::Result>
        + Present<read_all_resources::Result>,
{
    pub fn new(database: Arc<D>, presenter: P) -> Self {
        Self {
            database,
            presenter,
        }
    }

    fn owner_controller(&self) -> Controller<D, P> {
        Controller::new(&self.database, &self.presenter)
    }

    pub async fn create_owner(
        &self,
        name: &str,
    ) -> <P as Present<create_owner::Result>>::ViewModel {
        self.owner_controller().create_owner(name).await
    }

    pub async fn add_resource(
        &self,
        owner_id: &crate::domain::owner::OwnerId,
        name: &str,
        description: &str,
        price: crate::domain::owner::Price,
    ) -> <P as Present<add_resource::Result>>::ViewModel {
        self.owner_controller()
            .add_resource(owner_id, name, description, price)
            .await
    }

    pub async fn remove_resource(
        &self,
        owner_id: &crate::domain::owner::OwnerId,
        resource_id: &crate::domain::owner::ResourceId,
    ) -> <P as Present<remove_resource::Result>>::ViewModel {
        self.owner_controller()
            .remove_resource(owner_id, resource_id)
            .await
    }

    pub async fn read_all_resources(
        &self,
        owner_id: &OwnerId,
    ) -> <P as Present<read_all_resources::Result>>::ViewModel {
        self.owner_controller().read_all_resources(owner_id).await
    }
}
