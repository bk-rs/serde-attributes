pub mod syn;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Rename {
    Serialize(String),
    Deserialize(String),
    Both(String),
}
impl Rename {
    pub fn ser_name(&self) -> Option<&str> {
        match self {
            Self::Serialize(name) | Self::Both(name) => Some(name),
            _ => None,
        }
    }

    pub fn de_name(&self) -> Option<&str> {
        match self {
            Self::Deserialize(name) | Self::Both(name) => Some(name),
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
            Rename::Serialize(name.to_owned()).ser_name(),
            Some(name.as_str())
        );
        assert_eq!(Rename::Deserialize(name.to_owned()).ser_name(), None);
        assert_eq!(
            Rename::Both(name.to_owned()).ser_name(),
            Some(name.as_str())
        );

        assert_eq!(Rename::Serialize(name.to_owned()).de_name(), None);
        assert_eq!(
            Rename::Deserialize(name.to_owned()).de_name(),
            Some(name.as_str())
        );
        assert_eq!(Rename::Both(name.to_owned()).de_name(), Some(name.as_str()));
    }
}
