mod independent_both;
mod independent_only_deserialize;
mod independent_only_serialize;
mod normal;

//
//
//
use std::convert::TryFrom as _;

use serde_attributes::{Rename, RenameIndependent};

use super::{parse_darling_rename, parse_serde_meta};

#[test]
fn simple() {
    let input = include_str!("normal.rs");
    assert_eq!(
        Rename::try_from(&parse_serde_meta(input)).unwrap(),
        Rename::Normal("name".to_owned())
    );
    assert_eq!(
        parse_darling_rename(input).unwrap(),
        Rename::Normal("name".to_owned())
    );

    let input = include_str!("independent_only_serialize.rs");
    assert_eq!(
        Rename::try_from(&parse_serde_meta(input)).unwrap(),
        Rename::Independent(RenameIndependent::Serialize("ser_name".to_owned()))
    );
    assert_eq!(
        parse_darling_rename(input).unwrap(),
        Rename::Independent(RenameIndependent::Serialize("ser_name".to_owned()))
    );

    let input = include_str!("independent_only_deserialize.rs");
    assert_eq!(
        Rename::try_from(&parse_serde_meta(input)).unwrap(),
        Rename::Independent(RenameIndependent::Deserialize("de_name".to_owned()))
    );
    assert_eq!(
        parse_darling_rename(input).unwrap(),
        Rename::Independent(RenameIndependent::Deserialize("de_name".to_owned()))
    );

    let input = include_str!("independent_both.rs");
    assert_eq!(
        Rename::try_from(&parse_serde_meta(input)).unwrap(),
        Rename::Independent(RenameIndependent::Both {
            serialize: "ser_name".to_owned(),
            deserialize: "de_name".to_owned()
        })
    );
    assert_eq!(
        parse_darling_rename(input).unwrap(),
        Rename::Independent(RenameIndependent::Both {
            serialize: "ser_name".to_owned(),
            deserialize: "de_name".to_owned()
        })
    );
}
