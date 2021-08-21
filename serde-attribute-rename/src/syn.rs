use std::{convert::TryFrom, error, fmt};

use syn::{punctuated::Punctuated, Lit, Meta, NestedMeta, Token};

use crate::Rename;

/// [Official doc](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L23)
pub const RENAME: &str = "rename";
/// [Official doc](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L26)
pub const SERIALIZE: &str = "serialize";
/// [Official doc](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L14)
pub const DESERIALIZE: &str = "deserialize";

//
impl TryFrom<&Meta> for Rename {
    type Error = FromMetaError;

    fn try_from(meta: &Meta) -> Result<Self, Self::Error> {
        // https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L319-L333
        match meta {
            Meta::NameValue(ref m) => {
                if m.path.is_ident(RENAME) {
                    Self::try_from((&m.lit, RenameType::Both)).map_err(FromMetaError::FromLitError)
                } else {
                    Err(FromMetaError::MetaPathMismatch)
                }
            }
            Meta::List(ref m) => {
                if m.path.is_ident(RENAME) {
                    Self::try_from(&m.nested).map_err(FromMetaError::FromNestedMetaPunctuatedError)
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
    FromLitError(FromLitError),
    FromNestedMetaPunctuatedError(FromNestedMetaPunctuatedError),
}
impl fmt::Display for FromMetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for FromMetaError {}

//
enum RenameType {
    Serialize,
    Deserialize,
    Both,
}

impl TryFrom<(&Lit, RenameType)> for Rename {
    type Error = FromLitError;

    fn try_from((lit, tp): (&Lit, RenameType)) -> Result<Self, Self::Error> {
        match lit {
            Lit::Str(ref s) => Ok(match tp {
                RenameType::Serialize => Self::Serialize(s.value()),
                RenameType::Deserialize => Self::Deserialize(s.value()),
                RenameType::Both => Self::Both(s.value()),
            }),
            _ => Err(FromLitError::LitTypeMismatch),
        }
    }
}
#[derive(Debug)]
pub enum FromLitError {
    LitTypeMismatch,
}
impl fmt::Display for FromLitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for FromLitError {}

//
impl TryFrom<&Punctuated<NestedMeta, Token![,]>> for Rename {
    type Error = FromNestedMetaPunctuatedError;

    fn try_from(
        nested_meta_punctuated: &Punctuated<NestedMeta, Token![,]>,
    ) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
#[derive(Debug)]
pub enum FromNestedMetaPunctuatedError {}
impl fmt::Display for FromNestedMetaPunctuatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for FromNestedMetaPunctuatedError {}

//
impl TryFrom<&NestedMeta> for Rename {
    type Error = FromNestedMetaError;

    fn try_from(nested_meta: &NestedMeta) -> Result<Self, Self::Error> {
        match nested_meta {
            NestedMeta::Meta(Meta::NameValue(m)) => {
                if m.path.is_ident(SERIALIZE) {
                    Self::try_from((&m.lit, RenameType::Serialize))
                        .map_err(FromNestedMetaError::FromLitError)
                } else {
                    Err(FromNestedMetaError::NestedMetaPathMismatch)
                }
            }
            NestedMeta::Meta(Meta::NameValue(m)) => {
                if m.path.is_ident(DESERIALIZE) {
                    Self::try_from((&m.lit, RenameType::Deserialize))
                        .map_err(FromNestedMetaError::FromLitError)
                } else {
                    Err(FromNestedMetaError::NestedMetaPathMismatch)
                }
            }
            _ => Err(FromNestedMetaError::NestedMetaTypeMismatch),
        }
    }
}
#[derive(Debug)]
pub enum FromNestedMetaError {
    NestedMetaTypeMismatch,
    NestedMetaPathMismatch,
    FromLitError(FromLitError),
}
impl fmt::Display for FromNestedMetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for FromNestedMetaError {}
