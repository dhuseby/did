use indexmap::IndexMap;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;
use crate::fields::{ Context, Subject, string_or_list };

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint {
    #[serde(rename = "@context", 
            skip_serializing_if = "Context::is_missing", 
            deserialize_with = "string_or_list", 
            default)]
    pub context: Context,
    #[serde(skip_serializing_if = "Subject::is_missing", default)]
    pub id: Subject,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint")]
    pub endpoint: String,
    #[serde(flatten)]
    pub extra: IndexMap<String, Value>
}
