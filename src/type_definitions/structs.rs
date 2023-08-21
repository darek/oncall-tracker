use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionGroup {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub definitions: Option<Vec<TypeDefinitionDetails>>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionDetails {
    pub id: String,
    pub name: String,
    pub base: u32,
    pub multiplier: f32
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTypeDefinitionGroup {
    pub name: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionDetailRequest {
    pub name: String,
    pub base: u32,
    pub multiplier: f32
}

