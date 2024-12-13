use super::{error::DomainError, rule::Rule};

#[allow(dead_code)]
pub trait Entity {
    fn check_rule(rule: impl Rule) -> Result<(), DomainError> {
        if !rule.is_valid() {
            return Err(DomainError {
                message: rule.message(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod domain_entity_tests {
    use crate::domain::shared::rule::MockRule;

    use super::*;

    struct ConcreteEntity;

    impl Entity for ConcreteEntity {}

    #[test]
    fn test_check_rule_is_valid() {
        let mut rule = MockRule::new();
        rule.expect_is_valid().times(1).returning(|| true);
        let result = ConcreteEntity::check_rule(rule);
        assert!(result.is_ok())
    }

    #[test]
    fn test_check_rule_is_invalid() {
        let mut rule = MockRule::new();
        rule.expect_is_valid().times(1).returning(|| false);
        rule.expect_message()
            .times(1)
            .returning(|| format!("An error message"));
        let result = ConcreteEntity::check_rule(rule);
        assert!(result.is_err())
    }
}
