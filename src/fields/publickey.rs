use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_derive::{Deserialize, Serialize};
use std::default::Default;
use std::fmt;
use std::str::FromStr;
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

#[derive(Debug, PartialEq)]
pub enum PublicKeyDataType {
    Unknown,
    Pem,
    Jwk,
    Hex,
    Base64,
    Base58,
    Multibase,
    EthereumAddress
}

impl FromStr for PublicKeyDataType {
    type Err = ();

    fn from_str(s: &str) -> Result<PublicKeyDataType, Self::Err> {
        match s {
            "publicKeyUnknown" => Ok(PublicKeyDataType::Unknown),
            "publicKeyPem" => Ok(PublicKeyDataType::Pem),
            "publicKeyJwk" => Ok(PublicKeyDataType::Jwk),
            "publicKeyHex" => Ok(PublicKeyDataType::Hex),
            "publicKeyBase64" => Ok(PublicKeyDataType::Base64),
            "publicKeyBase58" => Ok(PublicKeyDataType::Base58),
            "publicKeyMultibase" => Ok(PublicKeyDataType::Multibase),
            "ethereumAddress" => Ok(PublicKeyDataType::EthereumAddress),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct PublicKey {
    pub id: Subject,
    pub key_type: PublicKeyType,
    pub controller: Subject,
    pub key_data_type: PublicKeyDataType,
    pub key_data: String,
    pub reference: bool
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Subject, Type, Controller, KeyData(PublicKeyDataType) };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "type",
                    "controller",
                    "publicKeyUnknown",
                    "publicKeyPem",
                    "publicKeyJwk",
                    "publicKeyHex",
                    "publicKeyBase64",
                    "publicKeyBase58",
                    "publicKeyMultibase",
                    "ethereumAddress" ];

                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`id`, `type`, `controller`, or one of the key types")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "id" => Ok(Field::Subject),
                            "type" => Ok(Field::Type),
                            "controller" => Ok(Field::Controller),
                            _ => {
                                if let Ok(pkdt) = PublicKeyDataType::from_str(value) {
                                    Ok(Field::KeyData(pkdt))
                                } else {
                                    Err(de::Error::unknown_field(value, FIELDS))
                                }
                            }
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct PublicKeyVisitor;

        impl<'de> Visitor<'de> for PublicKeyVisitor {
            type Value = PublicKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("DID string PublicKey struct")
            }

            fn visit_str<E>(self, value: &str) -> Result<PublicKey, E>
            where
                E: de::Error,
            {
                Ok(PublicKey {
                    id: Subject::new(value),
                    key_type: PublicKeyType::default(),
                    controller: Subject::default(),
                    key_data_type: PublicKeyDataType::Unknown,
                    key_data: "".to_owned(),
                    reference: true
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<PublicKey, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut subject = None;
                let mut key_type = None;
                let mut controller = None;
                let mut key_data_type = None;
                let mut key_data = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Subject => {
                            if subject.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            subject = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if key_type.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            key_type = Some(map.next_value()?);
                        }
                        Field::Controller => {
                            if controller.is_some() {
                                return Err(de::Error::duplicate_field("controller"));
                            }
                            controller = Some(map.next_value()?);
                        }
                        Field::KeyData(pkdt) => {
                            if key_data.is_some() {
                                return Err(de::Error::duplicate_field("key data"));
                            }
                            key_data_type = Some(pkdt);
                            key_data = Some(map.next_value()?);
                        }
                    }
                }

                let subject = subject.ok_or_else(|| de::Error::missing_field("id"))?;
                let key_type = key_type.ok_or_else(|| de::Error::missing_field("type"))?;
                let controller = controller.ok_or_else(|| de::Error::missing_field("controller"))?;
                let key_data_type = key_data_type.ok_or_else(|| de::Error::missing_field("key data"))?;
                let key_data = key_data.ok_or_else(|| de::Error::missing_field("key data"))?;

                Ok(PublicKey {
                    id: subject,
                    key_type: key_type,
                    controller: controller,
                    key_data_type: key_data_type,
                    key_data: key_data,
                    reference: false
                })
            }
        }

        deserializer.deserialize_any(PublicKeyVisitor)
    }
}
 
impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.reference {
            serializer.serialize_str(self.id.as_str())
        } else {
            let mut pk = serializer.serialize_struct("", 4)?;
            pk.serialize_field("id", &self.id)?;
            pk.serialize_field("type", &self.key_type)?;
            pk.serialize_field("controller", &self.controller)?;
            match self.key_data_type {
                PublicKeyDataType::Unknown => pk.serialize_field("publicKeyUnknown", &self.key_data)?,
                PublicKeyDataType::Pem => pk.serialize_field("publicKeyPem", &self.key_data)?,
                PublicKeyDataType::Jwk => pk.serialize_field("publicKeyJwk", &self.key_data)?,
                PublicKeyDataType::Hex => pk.serialize_field("publicKeyHex", &self.key_data)?,
                PublicKeyDataType::Base64 => pk.serialize_field("publicKeyBase64", &self.key_data)?,
                PublicKeyDataType::Base58 => pk.serialize_field("publicKeyBase58", &self.key_data)?,
                PublicKeyDataType::Multibase => pk.serialize_field("publicKeyMultibase", &self.key_data)?,
                PublicKeyDataType::EthereumAddress => pk.serialize_field("ethereumAddress", &self.key_data)?
            }
            pk.end()
        }
    }
}
