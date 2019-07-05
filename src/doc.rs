use indexmap::IndexMap;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;
use crate::{ Context, string_or_list };

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject(String);

impl Subject {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PublicKeyType {
    Ed25519VerificationKey2018,
    RsaVerificationKey2018,
    EcdsaSecp256k1VerificationKey2019
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum SignatureType {
    Ed25519Signature2018,
    RsaSignature2018,
    EcdsaKoblitzSignature2016,
    EcdsaSecp256k1Signature2019,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PublicKeyData {
    Pem { #[serde(rename = "publicKeyPem")] key: String },
    Jwk { #[serde(rename = "publicKeyJwk")] key: String },
    Hex { #[serde(rename = "publicKeyHex")] key: String },
    Base64 { #[serde(rename = "publicKeyBase64")] key: String },
    Base58 { #[serde(rename = "publicKeyBase58")] key: String },
    Multibase { #[serde(rename = "publicKeyMultibase")] key: String },
    EthAddr { #[serde(rename = "ethereumAddress")] key: String }
}

impl PublicKeyData {
    pub fn as_str(&self) -> &str {
        match self {
            PublicKeyData::Pem{ key } => &key,
            PublicKeyData::Jwk{ key } => &key,
            PublicKeyData::Hex{ key } => &key,
            PublicKeyData::Base64{ key } => &key,
            PublicKeyData::Base58{ key } => &key,
            PublicKeyData::Multibase{ key } => &key,
            PublicKeyData::EthAddr{ key } => &key,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    pub id: Subject,
    #[serde(rename = "type")]
    pub key_type: PublicKeyType,
    pub controller: Subject,
    #[serde(flatten)]
    pub key_data: PublicKeyData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthValue {
    pub id: Subject,
    #[serde(rename = "type")]
    pub auth_type: PublicKeyType,
    pub controller: Subject,
    #[serde(flatten)]
    pub key_data: PublicKeyData
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Authentication {
    Reference(Subject),
    Value(AuthValue)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint {
    #[serde(rename = "@context", 
            skip_serializing_if = "Context::is_missing", 
            deserialize_with = "string_or_list", 
            default)]
    pub context: Context,
    pub id: Subject,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint")]
    pub endpoint: String,
    #[serde(flatten)]
    pub extra: IndexMap<String, Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    #[serde(rename = "@context", deserialize_with = "string_or_list")]
    pub context: Context,
    pub id: Subject,
    #[serde(rename = "publicKey", skip_serializing_if = "Vec::is_empty", default)]
    pub public_key: Vec<PublicKey>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub authentication: Vec<Authentication>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub service: Vec<ServiceEndpoint>
}
