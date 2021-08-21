#[cfg(feature = "with-darling")]
pub mod darling;
#[cfg(feature = "with-syn")]
pub mod syn;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Rename {
    Normal(String),
    Independent(RenameIndependent),
}
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum RenameIndependent {
    Serialize(String),
    Deserialize(String),
    Both {
        serialize: String,
        deserialize: String,
    },
}
impl Rename {
    pub fn ser_name(&self) -> Option<&str> {
        match self {
            Self::Normal(name)
            | Self::Independent(RenameIndependent::Serialize(name))
            | Self::Independent(RenameIndependent::Both {
                serialize: name,
                deserialize: _,
            }) => Some(name),
            _ => None,
        }
    }

    pub fn de_name(&self) -> Option<&str> {
        match self {
            Self::Normal(name)
            | Self::Independent(RenameIndependent::Deserialize(name))
            | Self::Independent(RenameIndependent::Both {
                serialize: _,
                deserialize: name,
            }) => Some(name),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser_name_and_de_name() {
        let name = "foo".to_owned();

        assert_eq!(
            Rename::Normal(name.to_owned()).ser_name(),
            Some(name.as_str())
        );
        assert_eq!(
            Rename::Normal(name.to_owned()).de_name(),
            Some(name.as_str())
        );

        assert_eq!(
            Rename::Independent(RenameIndependent::Serialize(name.to_owned())).ser_name(),
            Some(name.as_str())
        );
        assert_eq!(
            Rename::Independent(RenameIndependent::Serialize(name.to_owned())).de_name(),
            None
        );

        assert_eq!(
            Rename::Independent(RenameIndependent::Deserialize(name.to_owned())).ser_name(),
            None
        );
        assert_eq!(
            Rename::Independent(RenameIndependent::Deserialize(name.to_owned())).de_name(),
            Some(name.as_str())
        );

        assert_eq!(
            Rename::Independent(RenameIndependent::Both {
                serialize: name.to_owned(),
                deserialize: name.to_owned(),
            })
            .ser_name(),
            Some(name.as_str())
        );
        assert_eq!(
            Rename::Independent(RenameIndependent::Both {
                serialize: name.to_owned(),
                deserialize: name.to_owned(),
            })
            .de_name(),
            Some(name.as_str())
        );
    }
}
