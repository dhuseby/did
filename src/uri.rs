use crate::{DidError, DidErrorKind};

use std::{
    collections::BTreeMap,
    default::Default,
    str::FromStr
};

use nom::{
    bytes::complete::{is_a, is_not, tag, take_while},
    character::complete::char,
    combinator::{map, map_res, opt},
    multi::separated_list,
    sequence::preceded,
    IResult,
};

use serde::{Serialize, Deserialize};
use serde::{de::{Deserializer, Visitor, Error}, ser::Serializer};

#[derive(Debug)]
pub struct Uri {
    empty: bool,
    pub id: String,
    pub method: String,
    pub path: Option<Vec<String>>,
    pub params: Option<BTreeMap<String, String>>,
    pub query: Option<BTreeMap<String, String>>,
    pub fragment: Option<String>,
}

impl Uri {
    pub fn new() -> Self {
        Uri::default()
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }
}

impl PartialEq<&str> for Uri {
    fn eq(&self, rhs: &&str) -> bool {
        let s = self.to_string();
        s == *rhs
    }
}

impl PartialEq<str> for Uri {
    fn eq(&self, rhs: &str) -> bool {
        let s = self.to_string();
        s == rhs
    }
}

impl PartialEq for Uri {
    fn eq(&self, rhs: &Uri) -> bool {
        let ls = self.to_string();
        let rs = rhs.to_string();
        ls == rs
    }
}

impl Default for Uri {
    fn default() -> Self {
        Uri {
            empty: true,
            id: String::default(),
            method: String::default(),
            path: None,
            params: None,
            query: None,
            fragment: None
        }
    }
}

impl FromStr for Uri {
    type Err = DidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_did_string(s.as_bytes()) {
            Ok((_, d)) => Ok(d),
            Err(_) => Err(DidError::from_kind(DidErrorKind::InvalidUri)),
        }
    }
}

impl Clone for Uri {
    fn clone(&self) -> Self {
        Uri {
            empty: self.empty,
            id: self.id.clone(),
            method: self.method.clone(),
            path: self.path.clone(),
            params: self.params.clone(),
            query: self.query.clone(),
            fragment: self.fragment.clone(),
        }
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.empty {
            return write!(f, "");
        }

        let mut path = String::new();
        if let Some(p) = &self.path {
            path.push('/');
            path.push_str(&p.join("/"));
        }

        let mut params = String::new();
        if let Some(p) = &self.params {
            if params.len() == 0 {
                params.push(';');
            }
            params.push_str(
                &p.iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<String>>()
                    .join(";"),
            );
        }
        let mut query = String::new();
        if let Some(q) = &self.query {
            query.push('?');
            query.push_str(
                &q.iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<String>>()
                    .join("&"),
            );
        }
        let mut fragment = String::new();
        if let Some(f) = &self.fragment {
            fragment = format!("#{}", f);
        }

        write!(
            f,
            "did:{}:{}{}{}{}{}",
            self.method, self.id, path, params, query, fragment
        )
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UriVisitor;

        impl<'de> Visitor<'de> for UriVisitor {
            type Value = Uri;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DID string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Uri, E>
            where
                E: Error,
            {
                match Uri::from_str(value) {
                    Ok(d) => Ok(d),
                    Err(e) => Err(Error::custom(e.to_string()))
                }
            }
        }

        deserializer.deserialize_any(UriVisitor)
    }
}

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(s.as_str())
    }
}

fn parse_did_string(i: &[u8]) -> IResult<&[u8], Uri> {
    if i.len() == 0 {
        return Ok((i, Uri {
            empty: true,
            id: String::default(),
            method: String::default(),
            path: None,
            params: None,
            query: None,
            fragment: None
        }));
    }

    let (i, _) = tag("did:")(i)?;
    let (i, method) = map(take_while(is_did_method_char), std::str::from_utf8)(i)?;
    let (i, _) = char(':')(i)?;
    let (i, id) = map(take_while(is_did_id_char), std::str::from_utf8)(i)?;
    let (i, path) = opt(did_path)(i)?;
    let (i, params) = opt(did_params)(i)?;
    let (i, query) = opt(did_query)(i)?;
    let (i, fragment) = opt(did_fragment)(i)?;

    Ok((
        i,
        Uri {
            empty: false,
            id: id.unwrap().to_string(),
            method: method.unwrap().to_string(),
            path: path.map(|s| { s.into_iter().map(|g| g.to_string()).collect() }),
            params: params.map(|m| {
                m.into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            }),
            query: query.map(|m| {
                m.into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            }),
            fragment: fragment.map(|s| s.to_string()),
        },
    ))
}
fn is_did_method_char(c: u8) -> bool {
    let c = c as char;
    c.is_ascii_lowercase() || c.is_ascii_digit()
}
fn is_did_id_char(c: u8) -> bool {
    let c = c as char;
    c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-'
}

fn did_path(i: &[u8]) -> IResult<&[u8], Vec<&str>> {
    let (i, segments) = preceded(char('/'), separated_list(char('/'), map_res(is_a("abcedfghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-%:@!$&'().*+,"), std::str::from_utf8)))(i)?;
    Ok((i, segments.into_iter().collect()))
}

fn did_params(i: &[u8]) -> IResult<&[u8], BTreeMap<&str, &str>> {
    let (i, lst) = preceded(char(';'), separated_list(char(';'), param_item))(i)?;

    Ok((i, lst.into_iter().collect()))
}
fn param_item(i: &[u8]) -> IResult<&[u8], (&str, &str)> {
    let (i, key) = param_token(i)?;
    let (i, _) = char('=')(i)?;
    let (i, val) = param_token(i)?;
    Ok((i, (key, val)))
}
fn param_token(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(
        is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789%.-_:"),
        std::str::from_utf8,
    )(i)
}

fn did_query(i: &[u8]) -> IResult<&[u8], BTreeMap<&str, &str>> {
    let (i, lst) = preceded(char('?'), separated_list(char('&'), query_item))(i)?;

    Ok((i, lst.into_iter().collect()))
}
fn query_item(i: &[u8]) -> IResult<&[u8], (&str, &str)> {
    let (i, key) = query_token(i)?;
    let (i, _) = char('=')(i)?;
    let (i, val) = query_token(i)?;
    Ok((i, (key, val)))
}
fn query_token(i: &[u8]) -> IResult<&[u8], &str> {
    map_res(is_not("&=:#[]"), std::str::from_utf8)(i)
}

fn did_fragment(i: &[u8]) -> IResult<&[u8], &str> {
    preceded(char('#'), map_res(is_not(":#[]"), std::str::from_utf8))(i)
}

#[cfg(test)]
mod resolve_method_tests {
    use super::*;

    #[test]
    fn test_did_params() {
        let p = b";a=b;c=d";
        let d = did_params(p).unwrap().1;
        assert_eq!(d.get("a"), Some(&"b"));
        assert_eq!(d.get("c"), Some(&"d"));
        let p = b";a=b";
        let d = did_params(p).unwrap().1;
        assert_eq!(d.get("a"), Some(&"b"));
        assert_eq!(d.get("c"), None);
    }

    #[test]
    fn test_did_fragment() {
        let fragment = b"#first-page";
        let d = did_fragment(fragment).unwrap().1;
        assert_eq!(d, std::str::from_utf8(&fragment[1..]).unwrap());
    }

    #[test]
    fn test_did_query() {
        let q = b"?a=b&c=d";
        let d = did_query(q).unwrap().1;
        assert_eq!(d.get("a"), Some(&"b"));
        assert_eq!(d.get("c"), Some(&"d"));
        let q = b"?%61=%62";
        let d = did_query(q).unwrap().1;
        assert_eq!(d.get("%61"), Some(&"%62"));
    }

    #[test]
    fn test_did_path() {
        let path = b"/spec/trust_ping/1.0/ping";
        let p = did_path(path).unwrap().1;
        assert_eq!(p, vec!["spec", "trust_ping", "1.0", "ping"].iter().map(|s| s.to_string()).collect::<Vec<String>>());
    }
}
