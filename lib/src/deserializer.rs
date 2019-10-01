use serde::{Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct PoemMap {
    #[serde(serialize_with = "round_serialize")]
    pub author: Vec<String>,
    #[serde(serialize_with = "round_serialize")]
    pub title: Vec<String>,
    #[serde(serialize_with = "round_serialize")]
    pub body: Vec<String>,
}

fn round_serialize<S>(x: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    s.serialize_str(x[0].as_str())
}