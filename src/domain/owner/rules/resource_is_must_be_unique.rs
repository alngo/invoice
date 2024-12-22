use crate::domain::{
    owner::{Resource, ResourceId},
    shared::Rule,
};

pub struct ResourceIdMustBeUnique<'a, 'b> {
    id: &'a ResourceId,
    owner_resources: &'b Vec<Resource>,
}

impl<'a, 'b> ResourceIdMustBeUnique<'a, 'b> {
    pub fn new(id: &'a ResourceId, owner_resources: &'b Vec<Resource>) -> Self {
        Self {
            id,
            owner_resources,
        }
    }
}

impl<'a, 'b> Rule for ResourceIdMustBeUnique<'a, 'b> {
    fn is_valid(&self) -> bool {
        !self
            .owner_resources
            .iter()
            .any(|resource| resource.id() == self.id)
    }

    fn message(&self) -> String {
        format!("Resource id {} must be unique", self.id)
    }
}
