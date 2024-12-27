mod price;

pub use price::Price;

pub type ResourceId = uuid::Uuid;

#[derive(Debug, Default, Clone)]
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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn price(&self) -> &Price {
        &self.price
    }
}
