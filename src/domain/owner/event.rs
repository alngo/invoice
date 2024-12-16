use super::{
    email::Email,
    resource::{Price, ResourceId},
};

pub enum OwnerEvent {
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
    NameUpdated {
        name: String,
    },
    EmailUpdated {
        email: Email,
    },
}
