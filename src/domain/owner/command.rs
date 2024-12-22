use super::{
    resource::{Price, ResourceId},
    OwnerId,
};

pub enum OwnerCommand {
    CreateOwner {
        id: OwnerId,
        name: String,
    },
    UpdateOwner {
        id: OwnerId,
        name: String,
    },
    AddResource {
        id: ResourceId,
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
