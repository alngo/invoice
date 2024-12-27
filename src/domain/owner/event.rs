use super::{resource::ResourceId, OwnerId, Price};

#[derive(Debug, Clone)]
pub enum OwnerEvent {
    OwnerCreated {
        id: OwnerId,
        name: String,
    },
    OwnerUpdated {
        name: String,
    },
    ResourceAdded {
        resource_id: ResourceId,
        name: String,
        description: String,
        price: Price,
    },
    ResourceUpdated {
        resource_id: ResourceId,
        name: String,
        description: String,
        price: Price,
    },
    ResourceRemoved {
        resource_id: ResourceId,
    },
}
