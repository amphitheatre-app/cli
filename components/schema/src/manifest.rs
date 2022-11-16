use super::Character;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest {
    pub character: Character,
}