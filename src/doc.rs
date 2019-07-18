use serde_derive::{Serialize, Deserialize};
use serde_json;
use std::str::FromStr;
use std::string::{String, ToString};
use void::Void;
use crate::fields::{
    Context, 
    PublicKey, 
    ServiceEndpoint, 
    Subject, 
    string_or_list,
};

#[derive(Serialize, Deserialize, Debug)]
struct ParsedDocument {
    #[serde(rename = "@context", deserialize_with = "string_or_list")]
    pub context: Context,
    pub id: Subject,
    #[serde(rename = "publicKey", skip_serializing_if = "Vec::is_empty", default)]
    pub public_key: Vec<PublicKey>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub authentication: Vec<PublicKey>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub service: Vec<ServiceEndpoint>
}

pub struct Document {
    doc: ParsedDocument
}

impl Document {
    pub fn new(context: &str, id: &str) -> Self {
        Document {
            doc: ParsedDocument { 
                context: Context::from_str(context).unwrap(),
                id: Subject::from_str(id).unwrap(),
                public_key: Vec::default(),
                authentication: Vec::default(),
                service: Vec::default()
            }
        }
    }
}

impl ToString for Document {
    fn to_string(&self) -> String {
        serde_json::to_string(&self.doc).unwrap()
    }
}

impl FromStr for Document {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Document { doc: serde_json::from_str(s).unwrap() })
    }
}

