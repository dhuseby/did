extern crate did_doc as did;

use did::{
    DidErrorKind,
    Uri,
};
use std::str::FromStr;

#[test]
fn did_uri_0() {
    let uri0 = Uri::new();
    let uri1 = Uri::default();
    let uri2 = uri0.clone();
    let uri3 = uri1.clone();

    // make sure they're empty
    assert!(uri0.is_empty());
    assert!(uri1.is_empty());
    assert!(uri2.is_empty());
    assert!(uri3.is_empty());

    // Uri comparisons
    assert_eq!(uri0, uri1);
    assert_eq!(uri0, uri2);
    assert_eq!(uri0, uri3);
    assert_eq!(uri1, uri2);
    assert_eq!(uri1, uri3);
    assert_eq!(uri2, uri3);

    // str comparisons
    assert_eq!(uri0, "");
    assert_eq!(uri1, "");
    assert_eq!(uri2, "");
    assert_eq!(uri3, "");

    // str comparisons
    let s = "";
    assert_eq!(uri0, s);
    assert_eq!(uri1, s);
    assert_eq!(uri2, s);
    assert_eq!(uri3, s);
}

#[test]
fn did_uri_1() {
    let did = Uri::from_str("did:git:akjsdhgaksdjhgasdkgh");
    assert!(did.is_ok());
    let did = did.unwrap();
    assert_eq!(did.id, "akjsdhgaksdjhgasdkgh".to_string());
    assert_eq!(did.method, "git".to_string());
    assert!(did.fragment.is_none());
    assert!(did.params.is_none());
    assert!(did.query.is_none());

    let did = Uri::from_str("did:git:");
    assert!(did.is_ok());
    let did = did.unwrap();
    assert_eq!(did.id, "".to_string());

    let did = Uri::from_str("did:sov:123456ygbvgfred;pool=mainnet;key=gdsadsfgdsfah");
    assert!(did.is_ok());
    let did = did.unwrap();
    assert!(did.params.is_some());
    let params = did.params.unwrap();
    assert_eq!(params.len(), 2);
    assert_eq!(params.get("pool"), Some(&"mainnet".to_string()));
    assert_eq!(params.get("key"), Some(&"gdsadsfgdsfah".to_string()));

    assert!(Uri::from_str("did:sov:builder:aksjdhgaksjdhgaskdgjh").is_ok());
    assert!(Uri::from_str("did:sov:test:aksjdhgaksjdhgaskdgjh").is_ok());
    let did = Uri::from_str(
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
fn did_uri_2() {
    for s in &["did:", "https://example.org", "did:git", "did:sov"] {
        let res = Uri::from_str(s);
        match res {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e.kind(), DidErrorKind::InvalidUri),
        };
    }
}

#[test]
fn did_uri_3() {
    let s = "did:sov:wjb4bjwb1235kbg1235/spec/tree/d7879f5e/text";
    let did = s.parse::<Uri>();
    assert!(did.is_ok());
    let did = did.unwrap();
    assert_eq!(did.id, "wjb4bjwb1235kbg1235");
    assert!(did.path.is_some());
    assert_eq!(did.path, Some(vec!["spec".to_string(), "tree".to_string(), "d7879f5e".to_string(), "text".to_string()]));
}
