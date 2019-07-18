use serde_derive::{Serialize, Deserialize};
use std::str::FromStr;
use void::Void;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[serde(transparent)]
pub struct Subject(String);

impl Subject {
    /*
    pub fn new(s: &str) -> Self {
        Subject(s.to_owned())
    }
    */

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_missing(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromStr for Subject {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Subject(s.to_owned()))
    }
}

impl PartialEq<&str> for Subject {
    fn eq(&self, rhs: &&str) -> bool {
        self.0 == *rhs
    }
}

impl PartialEq<str> for Subject {
    fn eq(&self, rhs: &str) -> bool {
        self.0 == rhs
    }
}

