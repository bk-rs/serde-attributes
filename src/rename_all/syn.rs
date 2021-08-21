use std::{convert::TryFrom, error, fmt};

use serde_rename_rule::{ParseError as RenameRuleParseError, RenameRule};
use syn::{Lit, Meta, MetaNameValue, NestedMeta};

use crate::rename::{syn::FromMetaError as RenameFromMetaError, Rename, RenameIndependent};

use super::{RenameAll, RenameAllIndependent};

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L24)
pub const RENAME_ALL: &str = "rename_all";

impl<'a> TryFrom<&'a Meta> for RenameAll {
    type Error = FromMetaError<'a>;

    fn try_from(meta: &'a Meta) -> Result<Self, Self::Error> {
        match Rename::try_from_meta(meta, RENAME_ALL) {
            Ok(rename) => match rename {
                Rename::Normal(name) => RenameRule::from_rename_all_str(name.as_str())
                    .map(Self::Normal)
                    .map_err(FromMetaError::RenameRuleParseError),
                Rename::Independent(RenameIndependent::Serialize(ser_name)) => {
                    RenameRule::from_rename_all_str(ser_name.as_str())
                        .map(|x| Self::Independent(RenameAllIndependent::Serialize(x)))
                        .map_err(FromMetaError::RenameRuleParseError)
                }
                Rename::Independent(RenameIndependent::Deserialize(de_name)) => {
                    RenameRule::from_rename_all_str(de_name.as_str())
                        .map(|x| Self::Independent(RenameAllIndependent::Deserialize(x)))
                        .map_err(FromMetaError::RenameRuleParseError)
                }
                Rename::Independent(RenameIndependent::Both {
                    serialize: ser_name,
                    deserialize: de_name,
                }) => Ok(Self::Independent(RenameAllIndependent::Both {
                    serialize: RenameRule::from_rename_all_str(ser_name.as_str())
                        .map_err(FromMetaError::RenameRuleParseError)?,
                    deserialize: RenameRule::from_rename_all_str(de_name.as_str())
                        .map_err(FromMetaError::RenameRuleParseError)?,
                })),
            },
            Err(err) => match err {
                RenameFromMetaError::MetaTypeOrPathMismatch(meta) => {
                    Err(FromMetaError::MetaTypeOrPathMismatch(meta))
                }
                RenameFromMetaError::LitTypeMismatch(lit) => {
                    Err(FromMetaError::LitTypeMismatch(lit))
                }
                RenameFromMetaError::NestedMetaTypeMismatch(nested_meta) => {
                    Err(FromMetaError::NestedMetaTypeMismatch(nested_meta))
                }
                RenameFromMetaError::NestedMetaPathMismatch(nested_meta, meta_name_value) => Err(
                    FromMetaError::NestedMetaPathMismatch(nested_meta, meta_name_value),
                ),
                RenameFromMetaError::AtLeastOneOfSerAndDe => {
                    Err(FromMetaError::AtLeastOneOfSerAndDe)
                }
            },
        }
    }
}

pub enum FromMetaError<'a> {
    MetaTypeOrPathMismatch(&'a Meta),
    LitTypeMismatch(&'a Lit),
    NestedMetaTypeMismatch(&'a NestedMeta),
    NestedMetaPathMismatch(&'a NestedMeta, &'a MetaNameValue),
    AtLeastOneOfSerAndDe,
    RenameRuleParseError(RenameRuleParseError),
}
impl<'a> fmt::Debug for FromMetaError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MetaTypeOrPathMismatch(_) => write!(f, "MetaTypeOrPathMismatch"),
            Self::LitTypeMismatch(_) => write!(f, "LitTypeMismatch"),
            Self::NestedMetaTypeMismatch(_) => write!(f, "NestedMetaTypeMismatch"),
            Self::NestedMetaPathMismatch(_, _) => write!(f, "NestedMetaPathMismatch"),
            Self::AtLeastOneOfSerAndDe => write!(f, "AtLeastOneOfSerAndDe"),
            Self::RenameRuleParseError(_) => write!(f, "RenameRuleParseError"),
        }
    }
}
impl<'a> fmt::Display for FromMetaError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<'a> error::Error for FromMetaError<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::{parse_str, DeriveInput, MetaList};

    fn to_meta(input: &str) -> Meta {
        match parse_str::<DeriveInput>(input)
            .unwrap()
            .attrs
            .first()
            .unwrap()
            .parse_meta()
            .unwrap()
        {
            Meta::List(MetaList {
                path,
                paren_token: _,
                nested,
            }) if path.is_ident("serde") => match nested.first().cloned() {
                Some(NestedMeta::Meta(meta)) => meta,
                _ => panic!(),
            },
            meta => {
                println!("{:?}", meta);
                panic!()
            }
        }
    }

    #[test]
    fn test_normal() {
        let input = r#"
        #[serde(rename_all = "snake_case")]
        pub struct Foo;
        "#;
        assert_eq!(
            RenameAll::try_from(&to_meta(input)).unwrap(),
            RenameAll::Normal(RenameRule::SnakeCase)
        );
    }

    #[test]
    fn test_independent_only_serialize() {
        let input = r#"
        #[serde(rename_all(serialize = "lowercase"))]
        pub struct Foo;
        "#;
        assert_eq!(
            RenameAll::try_from(&to_meta(input)).unwrap(),
            RenameAll::Independent(RenameAllIndependent::Serialize(RenameRule::LowerCase))
        );
    }

    #[test]
    fn test_independent_only_deserialize() {
        let input = r#"
        #[serde(rename_all(deserialize = "UPPERCASE"))]
        pub struct Foo;
        "#;
        assert_eq!(
            RenameAll::try_from(&to_meta(input)).unwrap(),
            RenameAll::Independent(RenameAllIndependent::Deserialize(RenameRule::UpperCase))
        );
    }

    #[test]
    fn test_independent_both() {
        let input = r#"
        #[serde(rename_all(serialize = "lowercase", deserialize = "UPPERCASE"))]
        pub struct Foo;
        "#;
        assert_eq!(
            RenameAll::try_from(&to_meta(input)).unwrap(),
            RenameAll::Independent(RenameAllIndependent::Both {
                serialize: RenameRule::LowerCase,
                deserialize: RenameRule::UpperCase
            })
        );
    }
}
