use crate::fields::{string_or_list, Context, Subject};
use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint {
    #[serde(
        rename = "@context",
        skip_serializing_if = "Context::is_empty",
        deserialize_with = "string_or_list",
        default
    )]
    context: Context,
    #[serde(skip_serializing_if = "Subject::is_empty", default)]
    id: Subject,
    #[serde(rename = "type")]
    service_type: String,
    #[serde(rename = "serviceEndpoint")]
    endpoint: String,
    #[serde(flatten)]
    pub extra: IndexMap<String, Value>,
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
