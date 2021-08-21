use std::{convert::TryFrom, error, fmt};

use syn::{punctuated::Punctuated, Lit, Meta, NestedMeta, Token};

use crate::Rename;

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
            Meta::NameValue(ref m) => {
                if m.path.is_ident(RENAME) {
                    match &m.lit {
                        Lit::Str(ref s) => Ok(Self::Normal(s.value())),
                        _ => Err(FromMetaError::LitTypeMismatch),
                    }
                } else {
                    Err(FromMetaError::MetaPathMismatch)
                }
            }
            Meta::List(ref m) => {
                if m.path.is_ident(RENAME) {
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

                    unimplemented!()
                } else {
                    Err(FromMetaError::MetaPathMismatch)
                }
            }
            _ => Err(FromMetaError::MetaTypeMismatch),
        }
    }
}
#[derive(Debug)]
pub enum FromMetaError {
    MetaTypeMismatch,
    MetaPathMismatch,
    LitTypeMismatch,
    NestedMetaTypeMismatch,
    NestedMetaPathMismatch,
}
impl fmt::Display for FromMetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for FromMetaError {}
