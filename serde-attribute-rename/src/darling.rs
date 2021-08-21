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
