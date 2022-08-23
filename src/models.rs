use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UriData {
    // #[serde(rename = "userId")]
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub seller_fee_basis_points: u16,
    pub image: String,
    pub attributes: Vec<Attribute>,
    pub properties: Properties,
    pub collection: Collection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

impl Attribute {
    pub fn update(&mut self, new_val: String) {
        self.value = new_val;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub creators: Vec<Creator>,
    pub files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creator {
    pub address: String,
    pub share: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub uri: String,
    #[serde(rename = "type")]
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub family: String,
}
