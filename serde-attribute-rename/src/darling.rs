use std::convert::TryFrom;

use darling_core::{Error as DarlingError, FromMeta};
use syn::Meta;

pub use crate::{syn::FromMetaError, Rename};

impl FromMeta for Rename {
    fn from_meta(meta: &Meta) -> Result<Self, DarlingError> {
        Self::try_from(meta).map_err(|err| match err {
            FromMetaError::MetaTypeOrPathMismatch(meta) => match meta {
                Meta::Path(_) => DarlingError::unexpected_type("Meta::Path"),
                Meta::List(m) => DarlingError::unknown_field_path(&m.path),
                Meta::NameValue(m) => DarlingError::unknown_field_path(&m.path),
            },
            FromMetaError::LitTypeMismatch(lit) => DarlingError::unexpected_lit_type(lit),
            FromMetaError::NestedMetaTypeMismatch(_) => DarlingError::unexpected_type("Meta::List"),
            FromMetaError::NestedMetaPathMismatch(m) => DarlingError::unknown_field_path(&m.path),
            FromMetaError::AtLeastOneOfSerAndDe => {
                DarlingError::custom("must be at least one the serialize and deserialize")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use darling::FromDeriveInput;
    use syn::parse_str;

    use crate::RenameIndependent;

    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(serde))]
    struct SerdeDerive {
        rename: Rename,
    }

    #[test]
    fn test_normal() {
        let input = r#"
        #[derive(SerdeDerive)]
        #[serde(rename = "name")]
        pub struct Foo;
        "#;
        let serde_derive = SerdeDerive::from_derive_input(&parse_str(input).unwrap()).unwrap();
        assert_eq!(serde_derive.rename, Rename::Normal("name".to_owned()));
    }

    #[test]
    fn test_independent_only_serialize() {
        let input = r#"
        #[derive(SerdeDerive)]
        #[serde(rename(serialize = "ser_name"))]
        pub struct Foo;
        "#;
        let serde_derive = SerdeDerive::from_derive_input(&parse_str(input).unwrap()).unwrap();
        assert_eq!(
            serde_derive.rename,
            Rename::Independent(RenameIndependent::Serialize("ser_name".to_owned()))
        );
    }

    #[test]
    fn test_independent_only_deserialize() {
        let input = r#"
        #[derive(SerdeDerive)]
        #[serde(rename(deserialize = "de_name"))]
        pub struct Foo;
        "#;
        let serde_derive = SerdeDerive::from_derive_input(&parse_str(input).unwrap()).unwrap();
        assert_eq!(
            serde_derive.rename,
            Rename::Independent(RenameIndependent::Deserialize("de_name".to_owned()))
        );
    }

    #[test]
    fn test_independent_both() {
        let input = r#"
        #[derive(SerdeDerive)]
        #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
        pub struct Foo;
        "#;
        let serde_derive = SerdeDerive::from_derive_input(&parse_str(input).unwrap()).unwrap();
        assert_eq!(
            serde_derive.rename,
            Rename::Independent(RenameIndependent::Both {
                serialize: "ser_name".to_owned(),
                deserialize: "de_name".to_owned()
            })
        );
    }
}
