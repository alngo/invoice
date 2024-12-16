use super::{
    email::Email,
    resource::{Price, ResourceId},
    OwnerId,
};

pub enum OwnerCommand {
    CreateOwner {
        name: String,
        email: Email,
    },
    UpdateOwner {
        id: OwnerId,
        name: String,
        email: Email,
    },
    AddResource {
        name: String,
        description: String,
        price: Price,
    },
    UpdateResource {
        resource_id: ResourceId,
        name: String,
        description: String,
        price: Price,
    },
    RemoveResource {
        resource_id: ResourceId,
    },
}
