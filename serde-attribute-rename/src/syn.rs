use std::{convert::TryFrom, error, fmt};

use syn::{Lit, Meta, NestedMeta};

use crate::{Rename, RenameIndependent};

/// [Official doc](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L23)
pub const RENAME: &str = "rename";
/// [Official doc](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L26)
pub const SERIALIZE: &str = "serialize";
/// [Official doc](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L14)
pub const DESERIALIZE: &str = "deserialize";

/// [Official doc](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L319-L333)
impl TryFrom<&Meta> for Rename {
    type Error = FromMetaError;

    fn try_from(meta: &Meta) -> Result<Self, Self::Error> {
        match meta {
            Meta::NameValue(ref m) if m.path.is_ident(RENAME) => match &m.lit {
                Lit::Str(ref s) => Ok(Self::Normal(s.value())),
                _ => Err(FromMetaError::LitTypeMismatch),
            },
            Meta::List(ref m) if m.path.is_ident(RENAME) => {
                let mut ser_name = None;
                let mut de_name = None;

                for nested_meta in &m.nested {
                    match nested_meta {
                        NestedMeta::Meta(Meta::NameValue(m)) => {
                            if m.path.is_ident(SERIALIZE) {
                                match &m.lit {
                                    Lit::Str(ref s) => ser_name = Some(s.value()),
                                    _ => return Err(FromMetaError::LitTypeMismatch),
                                }
                            } else if m.path.is_ident(DESERIALIZE) {
                                match &m.lit {
                                    Lit::Str(ref s) => de_name = Some(s.value()),
                                    _ => return Err(FromMetaError::LitTypeMismatch),
                                }
                            } else {
                                return Err(FromMetaError::NestedMetaPathMismatch);
                            }
                        }
                        _ => return Err(FromMetaError::NestedMetaTypeMismatch),
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
            _ => Err(FromMetaError::MetaTypeOrPathMismatch),
        }
    }
}
#[derive(Debug)]
pub enum FromMetaError {
    MetaTypeOrPathMismatch,
    LitTypeMismatch,
    NestedMetaTypeMismatch,
    NestedMetaPathMismatch,
    AtLeastOneOfSerAndDe,
}
impl fmt::Display for FromMetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for FromMetaError {}
