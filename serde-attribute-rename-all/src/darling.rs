use std::convert::TryFrom;

use darling_core::{Error as DarlingError, FromMeta};
use syn::Meta;

pub use crate::{syn::FromMetaError, RenameAll};

impl FromMeta for RenameAll {
    fn from_meta(meta: &Meta) -> Result<Self, DarlingError> {
        Self::try_from(meta).map_err(|err| match err {
            FromMetaError::MetaTypeOrPathMismatch(meta) => match meta {
                Meta::Path(_) => DarlingError::unexpected_type("Meta::Path"),
                Meta::List(meta_list) => DarlingError::unknown_field_path(&meta_list.path),
                Meta::NameValue(meta_name_value) => {
                    DarlingError::unknown_field_path(&meta_name_value.path)
                }
            },
            FromMetaError::LitTypeMismatch(lit) => DarlingError::unexpected_lit_type(lit),
            FromMetaError::NestedMetaTypeMismatch(_) => {
                DarlingError::unexpected_type("NestedMeta::Meta(!Meta::NameValue)")
            }
            FromMetaError::NestedMetaPathMismatch(_, meta_name_value) => {
                DarlingError::unknown_field_path(&meta_name_value.path)
            }
            FromMetaError::AtLeastOneOfSerAndDe => {
                DarlingError::custom("must be at least one the serialize and deserialize")
            }
            FromMetaError::RenameRuleParseError(err) => {
                DarlingError::custom(err.msg_for_rename_all())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use darling::FromDeriveInput;
    use serde_rename_rule::RenameRule;
    use syn::parse_str;

    use crate::RenameAllIndependent;

    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(serde))]
    struct SerdeDerive {
        rename_all: RenameAll,
    }

    fn to_serde_derive(input: &str) -> Result<SerdeDerive, DarlingError> {
        SerdeDerive::from_derive_input(&parse_str(input).unwrap())
    }

    #[test]
    fn test_normal() {
        let input = r#"
        #[derive(SerdeDerive)]
        #[serde(rename_all = "snake_case")]
        pub struct Foo;
        "#;
        assert_eq!(
            to_serde_derive(input).unwrap().rename_all,
            RenameAll::Normal(RenameRule::SnakeCase)
        );
    }

    #[test]
    fn test_independent_only_serialize() {
        let input = r#"
        #[derive(SerdeDerive)]
        #[serde(rename_all(serialize = "lowercase"))]
        pub struct Foo;
        "#;
        assert_eq!(
            to_serde_derive(input).unwrap().rename_all,
            RenameAll::Independent(RenameAllIndependent::Serialize(RenameRule::LowerCase))
        );
    }

    #[test]
    fn test_independent_only_deserialize() {
        let input = r#"
        #[derive(SerdeDerive)]
        #[serde(rename_all(deserialize = "UPPERCASE"))]
        pub struct Foo;
        "#;
        assert_eq!(
            to_serde_derive(input).unwrap().rename_all,
            RenameAll::Independent(RenameAllIndependent::Deserialize(RenameRule::UpperCase))
        );
    }

    #[test]
    fn test_independent_both() {
        let input = r#"
        #[derive(SerdeDerive)]
        #[serde(rename_all(serialize = "lowercase", deserialize = "UPPERCASE"))]
        pub struct Foo;
        "#;
        assert_eq!(
            to_serde_derive(input).unwrap().rename_all,
            RenameAll::Independent(RenameAllIndependent::Both {
                serialize: RenameRule::LowerCase,
                deserialize: RenameRule::UpperCase
            })
        );
    }
}
