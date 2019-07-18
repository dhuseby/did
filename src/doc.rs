use serde_derive::{Serialize, Deserialize};
use crate::fields::{
    Context, 
    PublicKey, 
    ServiceEndpoint, 
    Subject, 
    string_or_list,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
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
