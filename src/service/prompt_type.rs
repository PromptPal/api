use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PromptVariableTypes {
    String,
    Number,
    Boolean,
    Video,
    Audio,
    Image,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPromptItem {
    #[serde(rename = "id")]
    hash_id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "tokenCount")]
    token_count: i32,

    #[serde(rename = "variables")]
    variables: Vec<PromptVariable>,

    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptVariable {
    pub name: String,
    pub r#type: PromptVariableTypes,
}
