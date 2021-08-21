use std::convert::TryFrom;

use darling_core::{Error as DarlingError, FromMeta};
use syn::Meta;

use super::{syn::FromMetaError, Alias};

impl FromMeta for Alias {
    fn from_meta(meta: &Meta) -> Result<Self, DarlingError> {
        Self::try_from(meta).map_err(|err| match err {
            FromMetaError::MetaTypeOrPathMismatch(meta) => match meta {
                Meta::Path(_) => DarlingError::unexpected_type("Meta::Path"),
                Meta::List(_) => DarlingError::unexpected_type("Meta::List"),
                Meta::NameValue(meta_name_value) => {
                    DarlingError::unknown_field_path(&meta_name_value.path)
                }
            },
            FromMetaError::LitTypeMismatch(lit) => DarlingError::unexpected_lit_type(lit),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use darling::FromDeriveInput;
    use syn::parse_str;

    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(serde))]
    struct SerdeDerive {
        alias: Alias,
    }

    fn to_serde_derive(input: &str) -> Result<SerdeDerive, DarlingError> {
        SerdeDerive::from_derive_input(&parse_str(input).unwrap())
    }

    #[test]
    fn test_simple() {
        let input = r#"
        #[derive(SerdeDerive)]
        #[serde(alias = "name")]
        pub struct Foo;
        "#;
        assert_eq!(
            to_serde_derive(input).unwrap().alias,
            Alias("name".to_owned())
        );
    }
}
