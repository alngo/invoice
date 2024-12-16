use crate::domain::{owner::{Resource, ResourceId}, shared::rule::Rule};

pub struct OwnerMustOwnTheResource<'a, 'b> {
    resource_id: &'a ResourceId,
    owner_resources: &'b Vec<Resource>,
}

impl<'a, 'b> OwnerMustOwnTheResource<'a, 'b> {
    pub fn new(resource_id: &'a ResourceId, owner_resources: &'b Vec<Resource>) -> Self {
        Self {
            resource_id,
            owner_resources,
        }
    }
}

impl<'a, 'b> Rule for OwnerMustOwnTheResource<'a, 'b> {
    fn is_valid(&self) -> bool {
        self.owner_resources.iter().find(|resource| resource.id() == self.resource_id).is_some()
    }

    fn message(&self) -> String {
        format!(
            "Owner must own the resource, resource_id: {}",
            self.resource_id
        )
    }
}
