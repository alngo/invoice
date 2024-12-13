#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Rule {
    fn is_valid(&self) -> bool;
    fn message(&self) -> String;
}
