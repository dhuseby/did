use indexmap::IndexMap;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;
use crate::fields::{ Context, Subject, string_or_list };

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint {
    #[serde(rename = "@context", 
            skip_serializing_if = "Context::is_empty", 
            deserialize_with = "string_or_list", 
            default)]
    context: Context,
    #[serde(skip_serializing_if = "Subject::is_empty", default)]
    id: Subject,
    #[serde(rename = "type")]
    service_type: String,
    #[serde(rename = "serviceEndpoint")]
    endpoint: String,
    #[serde(flatten)]
    pub extra: IndexMap<String, Value>
}

impl ServiceEndpoint {

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn subject(&self) -> &Subject {
        &self.id
    }

    pub fn kind(&self) -> &String {
        &self.service_type
    }

    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }
}
