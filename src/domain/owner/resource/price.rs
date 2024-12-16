use rust_decimal::{prelude::FromPrimitive, Decimal};

use crate::domain::shared::error::DomainError;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Price(Decimal);

impl Price {
    pub fn from_f32(value: f32) -> Result<Self, DomainError> {
        if value < 0.0 {
            return Err(DomainError {
                message: format!("Price cannot be negative"),
            });
        }
        Ok(Self(Decimal::from_f32(value).unwrap()))
    }
}
