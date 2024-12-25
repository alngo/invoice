use crate::{
    adapter::shared::Present,
    application::owner::{add_resource, create_owner, read_all_resources, remove_resource},
};

#[derive(Default)]
pub struct Presenter;

impl Present<create_owner::Result> for Presenter {
    type ViewModel = String;

    fn present(&self, result: create_owner::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Owner created: {}", data.name),
            Err(err) => format!("Unable create owner: {err}"),
        }
    }
}

impl Present<add_resource::Result> for Presenter {
    type ViewModel = String;

    fn present(&self, result: add_resource::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Resource added: {}", data.resource_id),
            Err(err) => format!("Unable add resource: {err}"),
        }
    }
}

impl Present<remove_resource::Result> for Presenter {
    type ViewModel = String;

    fn present(&self, result: remove_resource::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Resource removed: {}", data.resource_id),
            Err(err) => format!("Unable remove resource: {err}"),
        }
    }
}

impl Present<read_all_resources::Result> for Presenter {
    type ViewModel = String;

    fn present(&self, result: read_all_resources::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Resources: {:#?}", data.resources),
            Err(err) => format!("Unable read resources: {err}"),
        }
    }
}
