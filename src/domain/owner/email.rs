use crate::domain::shared::error::DomainError;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn from_str(value: &str) -> Result<Self, DomainError> {
        if value.is_empty() {
            return Err(DomainError {
                message: format!("Email cannot be empty"),
            });
        }
        Ok(Self(value.to_string()))
    }
}
