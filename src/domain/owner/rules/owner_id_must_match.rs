use crate::domain::{owner::OwnerId, shared::rule::Rule};

pub struct OwnerIdMustMatch<'a, 'b> {
    left: &'a OwnerId,
    right: &'b OwnerId,
}

impl<'a, 'b> OwnerIdMustMatch<'a, 'b> {
    pub fn new(left: &'a OwnerId, right: &'b OwnerId) -> Self {
        Self { left, right }
    }
}

impl<'a, 'b> Rule for OwnerIdMustMatch<'a, 'b> {
    fn is_valid(&self) -> bool {
        self.left == self.right
    }

    fn message(&self) -> String {
        format!("OwnerId must match.",)
    }
}
