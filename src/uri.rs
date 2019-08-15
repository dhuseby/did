use crate::error::{DidError, DidErrorKind};

use std::{
    collections::HashMap,
    str::FromStr
};

use nom::{IResult,
          sequence::preceded,
          character::complete::char,
          multi::separated_list,
          combinator::{map_res, opt, map},
          bytes::complete::{is_a, is_not, tag, take_while}
};

#[derive(Debug)]
pub struct DidUri {
    pub id: String,
    pub method: String,
    pub params: Option<HashMap<String, String>>,
    pub query: Option<HashMap<String, String>>,
    pub fragment: Option<String>
}

impl FromStr for DidUri {
    type Err = DidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_did_string(s.as_bytes()) {
            Ok((_, d)) => Ok(d),
            Err(_) => Err(DidError::from_kind(DidErrorKind::InvalidDidUri))
        }
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

    Ok((i, DidUri { id: id.unwrap().to_string(),
                       method: method.unwrap().to_string(),
                       params: params.map(|m| m.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()),
                       query: query.map(|m| m.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()),
                       fragment: fragment.map(|s| s.to_string())
    }))
}
fn is_did_method_char(c: u8) -> bool {
    let c = c as char;
    c.is_ascii_lowercase() || c.is_ascii_digit()
}
fn is_did_id_char(c: u8) -> bool {
    let c = c as char;
    c.is_ascii_alphanumeric() ||
        c == '.' ||
        c == '_' ||
        c == '-'
}

fn did_params(i: &[u8]) -> IResult<&[u8], HashMap<&str, &str>> {
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
    map_res(is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789%.-_:"), std::str::from_utf8)(i)
}

fn did_query(i: &[u8]) -> IResult<&[u8], HashMap<&str, &str>> {
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
    }

    #[test]
    fn test_did_method_from_str_invalid() {
        for s in &["did:", "https://example.org", "did:git", "did:sov"] {
            let res = DidUri::from_str(s);
            match res {
                Ok(_) => assert!(false),
                Err(e) => assert_eq!(e.kind(), DidErrorKind::InvalidDidUri)
            };
        }
    }

}
