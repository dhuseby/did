use serde_derive::{Serialize, Deserialize};
use crate::fields::Subject;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PublicKeyType {
    UnknownKey,
    Ed25519VerificationKey2018,
    RsaVerificationKey2018,
    EcdsaSecp256k1VerificationKey2019
}

impl Default for PublicKeyType {
    fn default() -> Self {
        PublicKeyType::UnknownKey
    }
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
    Unknown { #[serde(rename = "publicKeyUnknown")] key: String },
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
            PublicKeyData::Unknown{ key } => &key,
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

impl Default for PublicKeyData {
    fn default() -> Self {
        PublicKeyData::Unknown{ key: "unknown".to_owned() }
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

