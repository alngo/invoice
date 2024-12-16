use super::{email::Email, resource::ResourceId, OwnerId, Price};

pub enum OwnerEvent {
    OwnerCreated {
        id: OwnerId,
        name: String,
        email: Email,
    },
    OwnerUpdated {
        name: String,
        email: Email,
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
