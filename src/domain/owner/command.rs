use super::{
    email::Email,
    resource::{Price, ResourceId},
};

pub enum OwnerCommand {
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
    UpdateName {
        name: String,
    },
    UpdateEmail {
        email: Email,
    },
}
