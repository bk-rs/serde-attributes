use std::{convert::TryFrom, error, fmt};

use syn::{Lit, Meta, MetaNameValue, NestedMeta};

use crate::{Rename, RenameIndependent};

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L23)
pub const RENAME: &str = "rename";
/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L26)
pub const SERIALIZE: &str = "serialize";
/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L14)
pub const DESERIALIZE: &str = "deserialize";

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L319-L333)
impl<'a> TryFrom<&'a Meta> for Rename {
    type Error = FromMetaError<'a>;

    fn try_from(meta: &'a Meta) -> Result<Self, Self::Error> {
        match meta {
            Meta::NameValue(ref meta_name_value) if meta_name_value.path.is_ident(RENAME) => {
                match &meta_name_value.lit {
                    Lit::Str(ref s) => Ok(Self::Normal(s.value())),
                    lit => Err(FromMetaError::LitTypeMismatch(lit)),
                }
            }
            Meta::List(ref meta_list) if meta_list.path.is_ident(RENAME) => {
                let mut ser_name = None;
                let mut de_name = None;

                for nested_meta in &meta_list.nested {
                    match nested_meta {
                        NestedMeta::Meta(Meta::NameValue(meta_name_value)) => {
                            if meta_name_value.path.is_ident(SERIALIZE) {
                                match &meta_name_value.lit {
                                    Lit::Str(ref s) => ser_name = Some(s.value()),
                                    _ => {
                                        return Err(FromMetaError::LitTypeMismatch(
                                            &meta_name_value.lit,
                                        ))
                                    }
                                }
                            } else if meta_name_value.path.is_ident(DESERIALIZE) {
                                match &meta_name_value.lit {
                                    Lit::Str(ref s) => de_name = Some(s.value()),
                                    _ => {
                                        return Err(FromMetaError::LitTypeMismatch(
                                            &meta_name_value.lit,
                                        ))
                                    }
                                }
                            } else {
                                return Err(FromMetaError::NestedMetaPathMismatch(
                                    nested_meta,
                                    meta_name_value,
                                ));
                            }
                        }
                        nested_meta => {
                            return Err(FromMetaError::NestedMetaTypeMismatch(nested_meta))
                        }
                    }
                }
                match (ser_name, de_name) {
                    (None, None) => Err(FromMetaError::AtLeastOneOfSerAndDe),
                    (None, Some(de_name)) => {
                        Ok(Self::Independent(RenameIndependent::Deserialize(de_name)))
                    }
                    (Some(ser_name), None) => {
                        Ok(Self::Independent(RenameIndependent::Serialize(ser_name)))
                    }
                    (Some(ser_name), Some(de_name)) => {
                        Ok(Self::Independent(RenameIndependent::Both {
                            serialize: ser_name,
                            deserialize: de_name,
                        }))
                    }
                }
            }
            meta => Err(FromMetaError::MetaTypeOrPathMismatch(meta)),
        }
    }
}

pub enum FromMetaError<'a> {
    MetaTypeOrPathMismatch(&'a Meta),
    LitTypeMismatch(&'a Lit),
    NestedMetaTypeMismatch(&'a NestedMeta),
    NestedMetaPathMismatch(&'a NestedMeta, &'a MetaNameValue),
    AtLeastOneOfSerAndDe,
}
impl<'a> fmt::Debug for FromMetaError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MetaTypeOrPathMismatch(_) => write!(f, "MetaTypeOrPathMismatch"),
            Self::LitTypeMismatch(_) => write!(f, "LitTypeMismatch"),
            Self::NestedMetaTypeMismatch(_) => write!(f, "NestedMetaTypeMismatch"),
            Self::NestedMetaPathMismatch(_, _) => write!(f, "NestedMetaPathMismatch"),
            Self::AtLeastOneOfSerAndDe => write!(f, "AtLeastOneOfSerAndDe"),
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

    use crate::RenameIndependent;

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
        #[serde(rename = "name")]
        pub struct Foo;
        "#;
        assert_eq!(
            Rename::try_from(&to_meta(input)).unwrap(),
            Rename::Normal("name".to_owned())
        );
    }

    #[test]
    fn test_independent_only_serialize() {
        let input = r#"
        #[serde(rename(serialize = "ser_name"))]
        pub struct Foo;
        "#;
        assert_eq!(
            Rename::try_from(&to_meta(input)).unwrap(),
            Rename::Independent(RenameIndependent::Serialize("ser_name".to_owned()))
        );
    }

    #[test]
    fn test_independent_only_deserialize() {
        let input = r#"
        #[serde(rename(deserialize = "de_name"))]
        pub struct Foo;
        "#;
        assert_eq!(
            Rename::try_from(&to_meta(input)).unwrap(),
            Rename::Independent(RenameIndependent::Deserialize("de_name".to_owned()))
        );
    }

    #[test]
    fn test_independent_both() {
        let input = r#"
        #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
        pub struct Foo;
        "#;
        assert_eq!(
            Rename::try_from(&to_meta(input)).unwrap(),
            Rename::Independent(RenameIndependent::Both {
                serialize: "ser_name".to_owned(),
                deserialize: "de_name".to_owned()
            })
        );
    }
}
