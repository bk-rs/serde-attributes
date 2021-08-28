#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
pub struct Foo {}
