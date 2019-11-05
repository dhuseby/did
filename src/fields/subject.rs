use serde_derive::{Deserialize, Serialize};
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use void::Void;
use crate::uri::Uri;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct Subject(Uri);

impl Subject {
    pub fn new(s: &str) -> Self {
        let did = Uri::from_str(s).unwrap();
        Subject(did)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Eq for Subject {}

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

impl PartialEq for Subject {
    fn eq(&self, rhs: &Subject) -> bool {
        self == rhs
    }
}

impl Hash for Subject {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let s = self.0.to_string();
        s.hash(state);
    }
}

impl FromStr for Subject {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Subject(Uri::from_str(s).unwrap()))
    }
}
