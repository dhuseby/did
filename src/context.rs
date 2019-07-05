use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_derive::Deserialize;
use std::default::Default;
use std::str::FromStr;
use void::Void;

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Context(Vec<String>);

impl Context {
    pub fn as_vec(&self) -> &Vec<String> {
        &self.0
    }

    pub fn is_missing(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Context {
    fn default() -> Self {
        Context(vec![])
    }
}

impl FromStr for Context {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Context(vec![s.to_owned()]))
    }
}


impl Serialize for Context
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0.len() {
            0 => serializer.serialize_none(),
            1 => serializer.serialize_str(&self.0[0]),
            _ => {
                let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
                for element in &self.0 {
                    seq.serialize_element(&element)?;
                }
                seq.end()
            }
        }
    }
}
