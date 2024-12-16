mod price;
mod repository;

pub use price::Price;

pub type ResourceId = uuid::Uuid;

#[derive(Debug, Default)]
pub struct Resource {
    id: ResourceId,
    name: String,
    description: String,
    price: Price,
}

impl Resource {
    pub fn new(id: ResourceId, name: String, description: String, price: Price) -> Self {
        Self {
            id,
            name,
            description,
            price,
        }
    }

    pub fn id(&self) -> &ResourceId {
        &self.id
    }
}
