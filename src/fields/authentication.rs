use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::default::Default;
use std::fmt;
use std::str::FromStr;
use void::Void;
use crate::{Subject, PublicKeyData, PublicKeyType};

#[derive(Debug)]
pub struct Authentication {
    pub id: Subject,
    pub auth_type: PublicKeyType,
    pub controller: Subject,
    pub key_data: String, //PublicKeyData,
    reference: bool
}

impl Authentication {
    pub fn is_reference(&self) -> bool {
        self.reference
    }
}

impl FromStr for Authentication {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Authentication {
            id: Subject::new(s),
            auth_type: PublicKeyType::default(),
            controller: Subject::default(),
            key_data: "".to_owned(), //PublicKeyData::default(),
            reference: true
        })
    }
}

impl<'de> Deserialize<'de> for Authentication {
    fn deserialize<D>(deserializer: D) -> Result<Authentication, D::Error>
    where
        D: Deserializer<'de>,
    {
		enum AuthField { 
            Id, 
            AuthType,
            Controller,
            Data(String) 
        };

        impl<'de> Deserialize<'de> for AuthField {
            fn deserialize<D>(deserializer: D) -> Result<AuthField, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = AuthField;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`id`, `type`, `controller`, or key data")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        println!("visiting key: {}", value);
                        match value {
                            "id" => Ok(AuthField::Id),
                            "type" => Ok(AuthField::AuthType),
                            "controller" => Ok(AuthField::Controller),
                            _ => Ok(AuthField::Data(value.to_owned())),
                            //_ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        // This is a Visitor that forwards string types to T's `FromStr` impl and
        // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
        // keep the compiler from complaining about T being an unused generic type
        // parameter. We need T in order to know the Value type for the Visitor
        // impl.
        //struct StringOrStruct<Authentication>(PhantomData<fn() -> Authentication>);
        struct AuthenticationVisitor;

        impl<'de> Visitor<'de> for AuthenticationVisitor {
            type Value = Authentication;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FromStr::from_str(value).unwrap())
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut id = None;
                let mut auth_type = None;
                let mut controller = None;
                let mut key_data = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        AuthField::Id => {
							if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        },
                        AuthField::AuthType => {
                            if auth_type.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            auth_type = Some(map.next_value()?);
                        },
                        AuthField::Controller => {
                            if controller.is_some() {
                                return Err(de::Error::duplicate_field("controller"));
                            }
                            controller = Some(map.next_value()?);
                        }
                        AuthField::Data(key) => {
                            if key_data.is_some() {
                                return Err(de::Error::duplicate_field("key data"));
                            }
                            println!("found key data with key: {}", key);
                            println!("-----");
                            key_data = Some(map.next_value()?);
                            println!("=====");
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let auth_type = auth_type.ok_or_else(|| de::Error::missing_field("type"))?;
                let controller = controller.ok_or_else(|| de::Error::missing_field("controller"))?;
                let key_data = key_data.ok_or_else(|| de::Error::missing_field("key data"))?;
                Ok(Authentication{
                    id: id,
                    auth_type: auth_type,
                    controller: controller,
                    key_data: key_data,
                    reference: false
                })
            }
        }

        deserializer.deserialize_any(AuthenticationVisitor)
    }
}

impl Serialize for Authentication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.reference {
            true => serializer.serialize_str(self.id.as_str()),
            false => {
                let mut auth = serializer.serialize_struct("", 4)?;
                auth.serialize_field("id", &self.id)?;
                auth.serialize_field("type", &self.auth_type)?;
                auth.serialize_field("controller", &self.controller)?;
                //TODO, field name is self.key_data.name...
                auth.serialize_field("key_data", &self.key_data)?;
                auth.end()
            }
        }
    }
}

