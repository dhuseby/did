use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_derive::{Deserialize, Serialize};
use std::default::Default;
use std::fmt;
use std::str::FromStr;
use crate::fields::Subject;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PublicKeyEncoding {
    Unknown,
    Pem,
    Jwk,
    Hex,
    Base64,
    Base58,
    Multibase,
    EthereumAddress
}

impl Default for PublicKeyEncoding {
    fn default() -> Self {
        PublicKeyEncoding::Unknown
    }
}

impl FromStr for PublicKeyEncoding {
    type Err = ();

    fn from_str(s: &str) -> Result<PublicKeyEncoding, Self::Err> {
        match s {
            "publicKeyUnknown" => Ok(PublicKeyEncoding::Unknown),
            "publicKeyPem" => Ok(PublicKeyEncoding::Pem),
            "publicKeyJwk" => Ok(PublicKeyEncoding::Jwk),
            "publicKeyHex" => Ok(PublicKeyEncoding::Hex),
            "publicKeyBase64" => Ok(PublicKeyEncoding::Base64),
            "publicKeyBase58" => Ok(PublicKeyEncoding::Base58),
            "publicKeyMultibase" => Ok(PublicKeyEncoding::Multibase),
            "ethereumAddress" => Ok(PublicKeyEncoding::EthereumAddress),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct PublicKey {
    id: Subject,
    key_type: PublicKeyType,
    controller: Subject,
    key_data_type: PublicKeyEncoding,
    key_data: String,
    reference: bool
}

impl PublicKey {

    pub fn subject(&self) -> &Subject {
        &self.id
    }

    pub fn controller(&self) -> &Subject {
        &self.controller
    }

    pub fn kind(&self) -> PublicKeyType {
        self.key_type
    }

    pub fn data(&self) -> &String {
        &self.key_data
    }

    pub fn encoding(&self) -> PublicKeyEncoding {
        self.key_data_type
    }

    pub fn reference(&self) -> bool {
        self.reference
    }
}


impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Subject, Type, Controller, KeyData(PublicKeyEncoding) };

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
                                if let Ok(pkdt) = PublicKeyEncoding::from_str(value) {
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
                    id: Subject::from_str(value).unwrap(),
                    key_type: PublicKeyType::default(),
                    controller: Subject::default(),
                    key_data_type: PublicKeyEncoding::Unknown,
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
                PublicKeyEncoding::Unknown => pk.serialize_field("publicKeyUnknown", &self.key_data)?,
                PublicKeyEncoding::Pem => pk.serialize_field("publicKeyPem", &self.key_data)?,
                PublicKeyEncoding::Jwk => pk.serialize_field("publicKeyJwk", &self.key_data)?,
                PublicKeyEncoding::Hex => pk.serialize_field("publicKeyHex", &self.key_data)?,
                PublicKeyEncoding::Base64 => pk.serialize_field("publicKeyBase64", &self.key_data)?,
                PublicKeyEncoding::Base58 => pk.serialize_field("publicKeyBase58", &self.key_data)?,
                PublicKeyEncoding::Multibase => pk.serialize_field("publicKeyMultibase", &self.key_data)?,
                PublicKeyEncoding::EthereumAddress => pk.serialize_field("ethereumAddress", &self.key_data)?
            }
            pk.end()
        }
    }
}
