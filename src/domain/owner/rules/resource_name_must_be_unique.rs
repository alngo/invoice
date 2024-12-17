use crate::domain::{owner::Resource, shared::Rule};

pub struct ResourceNameMustBeUnique<'a, 'b> {
    name: &'a String,
    owner_resources: &'b Vec<Resource>,
}

impl<'a, 'b> ResourceNameMustBeUnique<'a, 'b> {
    pub fn new(name: &'a String, owner_resources: &'b Vec<Resource>) -> Self {
        Self {
            name,
            owner_resources,
        }
    }
}

impl<'a, 'b> Rule for ResourceNameMustBeUnique<'a, 'b> {
    fn is_valid(&self) -> bool {
        !self
            .owner_resources
            .iter()
            .any(|resource| resource.name() == self.name)
    }

    fn message(&self) -> String {
        format!("Resource name {} must be unique", self.name)
    }
}
