#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "UPPERCASE"))]
pub struct Foo {}
