mod independent_both;
mod independent_only_deserialize;
mod independent_only_serialize;
mod normal;

//
//
//
use std::convert::TryFrom as _;

use serde_attributes::{
    rename_all::serde_rename_rule::RenameRule, RenameAll, RenameAllIndependent,
};

use super::{parse_darling_rename_all, parse_serde_meta};

#[test]
fn simple() {
    let input = include_str!("normal.rs");
    assert_eq!(
        RenameAll::try_from(&parse_serde_meta(input)).unwrap(),
        RenameAll::Normal(RenameRule::SnakeCase)
    );
    assert_eq!(
        parse_darling_rename_all(input).unwrap(),
        RenameAll::Normal(RenameRule::SnakeCase)
    );

    let input = include_str!("independent_only_serialize.rs");
    assert_eq!(
        RenameAll::try_from(&parse_serde_meta(input)).unwrap(),
        RenameAll::Independent(RenameAllIndependent::Serialize(RenameRule::LowerCase))
    );
    assert_eq!(
        parse_darling_rename_all(input).unwrap(),
        RenameAll::Independent(RenameAllIndependent::Serialize(RenameRule::LowerCase))
    );

    let input = include_str!("independent_only_deserialize.rs");
    assert_eq!(
        RenameAll::try_from(&parse_serde_meta(input)).unwrap(),
        RenameAll::Independent(RenameAllIndependent::Deserialize(RenameRule::UpperCase))
    );
    assert_eq!(
        parse_darling_rename_all(input).unwrap(),
        RenameAll::Independent(RenameAllIndependent::Deserialize(RenameRule::UpperCase))
    );

    let input = include_str!("independent_both.rs");
    assert_eq!(
        RenameAll::try_from(&parse_serde_meta(input)).unwrap(),
        RenameAll::Independent(RenameAllIndependent::Both {
            serialize: RenameRule::LowerCase,
            deserialize: RenameRule::UpperCase
        })
    );
    assert_eq!(
        parse_darling_rename_all(input).unwrap(),
        RenameAll::Independent(RenameAllIndependent::Both {
            serialize: RenameRule::LowerCase,
            deserialize: RenameRule::UpperCase
        })
    );
}
