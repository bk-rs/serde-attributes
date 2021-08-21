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
            Meta::NameValue(ref m) if m.path.is_ident(RENAME) => match &m.lit {
                Lit::Str(ref s) => Ok(Self::Normal(s.value())),
                _ => Err(FromMetaError::LitTypeMismatch(&m.lit)),
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
                                    _ => return Err(FromMetaError::LitTypeMismatch(&m.lit)),
                                }
                            } else if m.path.is_ident(DESERIALIZE) {
                                match &m.lit {
                                    Lit::Str(ref s) => de_name = Some(s.value()),
                                    _ => return Err(FromMetaError::LitTypeMismatch(&m.lit)),
                                }
                            } else {
                                return Err(FromMetaError::NestedMetaPathMismatch(m));
                            }
                        }
                        nm => return Err(FromMetaError::NestedMetaTypeMismatch(nm)),
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
            m => Err(FromMetaError::MetaTypeOrPathMismatch(m)),
        }
    }
}

pub enum FromMetaError<'a> {
    MetaTypeOrPathMismatch(&'a Meta),
    LitTypeMismatch(&'a Lit),
    NestedMetaTypeMismatch(&'a NestedMeta),
    NestedMetaPathMismatch(&'a MetaNameValue),
    AtLeastOneOfSerAndDe,
}
impl<'a> fmt::Debug for FromMetaError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MetaTypeOrPathMismatch(_) => write!(f, "MetaTypeOrPathMismatch"),
            Self::LitTypeMismatch(_) => write!(f, "LitTypeMismatch"),
            Self::NestedMetaTypeMismatch(_) => write!(f, "NestedMetaTypeMismatch"),
            Self::NestedMetaPathMismatch(_) => write!(f, "NestedMetaPathMismatch"),
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
