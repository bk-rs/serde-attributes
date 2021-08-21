pub use serde_rename_rule;

use serde_rename_rule::RenameRule;

pub mod syn;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum RenameAll {
    Serialize(RenameRule),
    Deserialize(RenameRule),
    Both(RenameRule),
}
impl RenameAll {
    pub fn ser_rule(&self) -> Option<&RenameRule> {
        match self {
            Self::Serialize(rule) | Self::Both(rule) => Some(rule),
            _ => None,
        }
    }

    pub fn de_rule(&self) -> Option<&RenameRule> {
        match self {
            Self::Deserialize(rule) | Self::Both(rule) => Some(rule),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser_rule_and_de_rule() {
        assert_eq!(
            RenameAll::Serialize(RenameRule::LowerCase).ser_rule(),
            Some(&RenameRule::LowerCase)
        );
        assert_eq!(
            RenameAll::Deserialize(RenameRule::LowerCase).ser_rule(),
            None
        );
        assert_eq!(
            RenameAll::Both(RenameRule::LowerCase).ser_rule(),
            Some(&RenameRule::LowerCase)
        );

        assert_eq!(RenameAll::Serialize(RenameRule::LowerCase).de_rule(), None);
        assert_eq!(
            RenameAll::Deserialize(RenameRule::LowerCase).de_rule(),
            Some(&RenameRule::LowerCase)
        );
        assert_eq!(
            RenameAll::Both(RenameRule::LowerCase).de_rule(),
            Some(&RenameRule::LowerCase)
        );
    }
}
