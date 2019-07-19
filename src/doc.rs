use indexmap::IndexMap;
use serde_derive::{Serialize, Deserialize};
use serde_json::{self, Value};
use std::collections::HashMap;
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
    context: Context,
    id: Subject,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    created: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    updated: String,
    #[serde(rename = "publicKey", skip_serializing_if = "Vec::is_empty", default)]
    public_key: Vec<PublicKey>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    authentication: Vec<PublicKey>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    service: Vec<ServiceEndpoint>,
    //#[serde(skip_serializing_if = "Proof::is_empty", default)]
    //pub proof: Proof,
    #[serde(flatten)]
    pub extra: IndexMap<String, Value>
}

pub struct Document {
    doc: ParsedDocument,
    public_keys: HashMap<Subject, PublicKey>,
    authentications: HashMap<Subject, PublicKey>,
    service_endpoints: HashMap<Subject, ServiceEndpoint>
}

impl Document {
    pub fn new(context: &str, id: &str) -> Self {
        Document {
            doc: ParsedDocument { 
                context: Context::from_str(context).unwrap(),
                id: Subject::from_str(id).unwrap(),
                created: String::new(),
                updated: String::new(),
                public_key: Vec::default(),
                authentication: Vec::default(),
                service: Vec::default(),
                extra: IndexMap::default()
            },
            public_keys: HashMap::new(),
            authentications: HashMap::new(),
            service_endpoints: HashMap::new()
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
        let doc = Document { 
            doc: serde_json::from_str(s).unwrap(),
            public_keys: HashMap::new(),
            authentications: HashMap::new(),
            service_endpoints: HashMap::new()
        };

        //TODO: initialize the public_keys, authentications, and
        //      service_endpoints HashMaps from the underlying Vecs in the
        //      parsed doc.

        Ok(doc)
    }
}

