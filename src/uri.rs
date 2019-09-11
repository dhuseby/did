use crate::error::{DidError, DidErrorKind};

use std::{collections::BTreeMap, str::FromStr};

use nom::{
    bytes::complete::{is_a, is_not, tag, take_while},
    character::complete::char,
    combinator::{map, map_res, opt},
    multi::separated_list,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
pub struct DidUri {
    pub id: String,
    pub method: String,
    pub params: Option<BTreeMap<String, String>>,
    pub query: Option<BTreeMap<String, String>>,
    pub fragment: Option<String>,
}

impl FromStr for DidUri {
    type Err = DidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_did_string(s.as_bytes()) {
            Ok((_, d)) => Ok(d),
            Err(_) => Err(DidError::from_kind(DidErrorKind::InvalidDidUri)),
        }
    }
}

impl Clone for DidUri {
    fn clone(&self) -> Self {
        DidUri {
            id: self.id.clone(),
            method: self.method.clone(),
            params: self.params.clone(),
            query: self.query.clone(),
            fragment: self.fragment.clone(),
        }
    }
}

impl std::fmt::Display for DidUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut params = String::new();
        if let Some(p) = &self.params {
            params.push(';');
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
            "did:{}:{}{}{}{}",
            self.method, self.id, params, query, fragment
        )
    }
}

fn parse_did_string(i: &[u8]) -> IResult<&[u8], DidUri> {
    let (i, _) = tag("did:")(i)?;
    let (i, method) = map(take_while(is_did_method_char), std::str::from_utf8)(i)?;
    let (i, _) = char(':')(i)?;
    let (i, id) = map(take_while(is_did_id_char), std::str::from_utf8)(i)?;
    let (i, params) = opt(did_params)(i)?;
    let (i, query) = opt(did_query)(i)?;
    let (i, fragment) = opt(did_fragment)(i)?;

    Ok((
        i,
        DidUri {
            id: id.unwrap().to_string(),
            method: method.unwrap().to_string(),
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

    use crate::error::DidErrorKind;

    #[test]
    fn test_did_method_from_str_valid() {
        let did = DidUri::from_str("did:git:akjsdhgaksdjhgasdkgh");
        assert!(did.is_ok());
        let did = did.unwrap();
        assert_eq!(did.id, "akjsdhgaksdjhgasdkgh".to_string());
        assert_eq!(did.method, "git".to_string());
        assert!(did.fragment.is_none());
        assert!(did.params.is_none());
        assert!(did.query.is_none());

        let did = DidUri::from_str("did:git:");
        assert!(did.is_ok());
        let did = did.unwrap();
        assert_eq!(did.id, "".to_string());

        let did = DidUri::from_str("did:sov:123456ygbvgfred;pool=mainnet;key=gdsadsfgdsfah");
        assert!(did.is_ok());
        let did = did.unwrap();
        assert!(did.params.is_some());
        let params = did.params.unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("pool"), Some(&"mainnet".to_string()));
        assert_eq!(params.get("key"), Some(&"gdsadsfgdsfah".to_string()));

        assert!(DidUri::from_str("did:sov:builder:aksjdhgaksjdhgaskdgjh").is_ok());
        assert!(DidUri::from_str("did:sov:test:aksjdhgaksjdhgaskdgjh").is_ok());
        let did = DidUri::from_str(
            "did:git:12345678jhasdg;file=Users_janedoe_.git?key=ham&value=meat#1-2-3",
        );

        assert!(did.is_ok());
        let did = did.unwrap();
        assert!(did.params.is_some());
        assert!(did.query.is_some());
        assert!(did.fragment.is_some());
        let params = &did.params.clone().unwrap();
        let query = &did.query.clone().unwrap();

        assert_eq!(params.get("file"), Some(&"Users_janedoe_.git".to_string()));
        assert_eq!(query.get("key"), Some(&"ham".to_string()));
        assert_eq!(query.get("value"), Some(&"meat".to_string()));
        assert_eq!(&did.fragment.clone().unwrap(), &"1-2-3".to_string());
        assert_eq!(
            did.to_string(),
            "did:git:12345678jhasdg;file=Users_janedoe_.git?key=ham&value=meat#1-2-3".to_string()
        );
    }

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
    fn test_did_method_from_str_invalid() {
        for s in &["did:", "https://example.org", "did:git", "did:sov"] {
            let res = DidUri::from_str(s);
            match res {
                Ok(_) => assert!(false),
                Err(e) => assert_eq!(e.kind(), DidErrorKind::InvalidDidUri),
            };
        }
    }

}
