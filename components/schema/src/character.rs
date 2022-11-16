use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Character {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: String,
    pub readme: String,
    pub homepage: String,
    pub repository: String,
    pub license: String,
    pub license_file: String,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
    pub publish: Vec<String>,
}