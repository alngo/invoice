use crate::{adapter::shared::Present, application::owner::create_owner};


pub struct Presenter {}

impl Present<create_owner::Result> for Presenter {
    type ViewModel = String;

    fn present(&self, result: create_owner::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Owner created: {}", data.name),
            Err(err) => format!("Unable create owner: {err}"),
        }
    }
}
