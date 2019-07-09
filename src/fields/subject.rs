use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[serde(transparent)]
pub struct Subject(String);

impl Subject {
    pub fn new(s: &str) -> Self {
        Subject(s.to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
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

