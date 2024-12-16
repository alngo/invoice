use super::{
    resource::{Price, ResourceId},
    OwnerId,
};

pub enum OwnerCommand {
    CreateOwner {
        name: String,
    },
    UpdateOwner {
        id: OwnerId,
        name: String,
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
