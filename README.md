# DID

[![Build Status](https://travis-ci.com/mikelodder7/did.svg?branch=master)](https://travis-ci.org/mikelodder7/did)

This is a Rust crate for working with DID documents as defined in the
[Decentralized Identifier Spec](https://w3c-ccg.github.io/did-spec/).

This crate currently supports two functions: parsing and verifying DID URIs and
DID Documents.  It does not handle DID method specs which are more specific to
a network or context.

The namespace is `did_uri` with top level re-exports of `Uri`, `Document`,
`DidError`, `DidErrorKind`.

Example of parsing a DID Uri:
```rust
use did_doc::Uri;


fn main() {
    //Valid DID URI
    let did = Uri::from_str("did:git:akjsdhgaksdjhgasdkgh").unwrap();
    
    //Invalid DID URI
    let res = Uri::from_str("did:git:");
    assert!(res.is_err());

    //Convert back to string
    let did_str = did.to_string();
}
```

Example of parsing a DID Document:
```rust
use did_doc::{
    fields::{PublicKeyEncoding, PublicKeyType},
    Document
};

fn main() {
    let jstr = r#"
    {
        "@context": "https://w3id.org/did/v1",
        "id": "did:example:123456789abcdefghi"
    }
    "#;

    //Valid did document;
    let doc = Document::from_str(&jstr).unwrap();    

    assert_eq!(doc.context().len(), 1);
    assert_eq!(doc.subject(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key().len(), 0);
    assert_eq!(doc.authentication().len(), 0);
    assert_eq!(doc.service().len(), 0);
}
```
