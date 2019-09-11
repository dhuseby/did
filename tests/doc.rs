extern crate did_doc as did;

use did::{
    fields::{PublicKeyEncoding, PublicKeyType},
    Document,
};

use std::str::FromStr;
use std::string::ToString;

#[test]
fn did_document_create_0() {
    let flat = r#"{"@context":"https://w3id.org/did/v1","id":"did:example:123456789abcdefghi"}"#;
    let doc = Document::new("https://w3id.org/did/v1", "did:example:123456789abcdefghi");
    let s = doc.to_string();
    assert_eq!(s.as_str(), flat);
}

#[test]
fn did_parse_document_0() {
    let jstr = r#"
    {
        "@context": "https://w3id.org/did/v1",
        "id": "did:example:123456789abcdefghi"
    }
    "#;

    let flat = r#"{"@context":"https://w3id.org/did/v1","id":"did:example:123456789abcdefghi"}"#;

    let doc = Document::from_str(&jstr).unwrap();
    assert_eq!(doc.context().len(), 1);
    assert_eq!(doc.subject(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key().len(), 0);
    assert_eq!(doc.authentication().len(), 0);
    assert_eq!(doc.service().len(), 0);

    let s = doc.to_string();
    assert_eq!(s.as_str(), flat);
}

#[test]
fn did_parse_document_1() {
    let jstr = r#"
    {
        "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
        "id": "did:example:123456789abcdefghi"
    }
    "#;

    let flat = r#"{"@context":["https://w3id.org/did/v1","https://w3id.org/security/v1"],"id":"did:example:123456789abcdefghi"}"#;

    let doc = Document::from_str(&jstr).unwrap();
    assert_eq!(doc.context().len(), 2);
    assert_eq!(doc.subject(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key().len(), 0);
    assert_eq!(doc.authentication().len(), 0);
    assert_eq!(doc.service().len(), 0);

    let s = doc.to_string();
    assert_eq!(s.as_str(), flat);
}

#[test]
fn did_parse_document_2() {
    let jstr = r#"
    {
        "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
        "id": "did:example:123456789abcdefghi",
        "publicKey": [{
            "id": "did:example:123456789abcdefghi#keys-1",
            "type": "RsaVerificationKey2018",
            "controller": "did:example:123456789abcdefghi",
            "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
        }, {
            "id": "did:example:123456789abcdefghi#keys-2",
            "type": "Ed25519VerificationKey2018",
            "controller": "did:example:pqrstuvwxyz0987654321",
            "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }, {
            "id": "did:example:123456789abcdefghi#keys-3",
            "type": "EcdsaSecp256k1VerificationKey2019",
            "controller": "did:example:123456789abcdefghi",
            "publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
        }]
    }
    "#;

    let flat = r#"{"@context":["https://w3id.org/did/v1","https://w3id.org/security/v1"],"id":"did:example:123456789abcdefghi","publicKey":[{"id":"did:example:123456789abcdefghi#keys-1","type":"RsaVerificationKey2018","controller":"did:example:123456789abcdefghi","publicKeyPem":"-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"},{"id":"did:example:123456789abcdefghi#keys-2","type":"Ed25519VerificationKey2018","controller":"did:example:pqrstuvwxyz0987654321","publicKeyBase58":"H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"},{"id":"did:example:123456789abcdefghi#keys-3","type":"EcdsaSecp256k1VerificationKey2019","controller":"did:example:123456789abcdefghi","publicKeyHex":"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"}]}"#;

    let doc = Document::from_str(&jstr).unwrap();
    assert_eq!(doc.context().len(), 2);
    assert_eq!(doc.subject(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key().len(), 3);
    assert_eq!(doc.authentication().len(), 0);
    assert_eq!(doc.service().len(), 0);

    let k1 = &doc.public_key()[0];
    assert_eq!(k1.subject(), "did:example:123456789abcdefghi#keys-1");
    assert_eq!(k1.kind(), PublicKeyType::RsaVerificationKey2018);
    assert_eq!(k1.controller(), "did:example:123456789abcdefghi");
    assert_eq!(k1.reference(), false);
    assert_eq!(k1.encoding(), PublicKeyEncoding::Pem);
    assert_eq!(
        k1.data().as_str(),
        "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
    );

    let k2 = &doc.public_key()[1];
    assert_eq!(k2.subject(), "did:example:123456789abcdefghi#keys-2");
    assert_eq!(k2.kind(), PublicKeyType::Ed25519VerificationKey2018);
    assert_eq!(k2.controller(), "did:example:pqrstuvwxyz0987654321");
    assert_eq!(k2.reference(), false);
    assert_eq!(k2.encoding(), PublicKeyEncoding::Base58);
    assert_eq!(
        k2.data().as_str(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
    );

    let k3 = &doc.public_key()[2];
    assert_eq!(k3.subject(), "did:example:123456789abcdefghi#keys-3");
    assert_eq!(k3.kind(), PublicKeyType::EcdsaSecp256k1VerificationKey2019);
    assert_eq!(k3.controller(), "did:example:123456789abcdefghi");
    assert_eq!(k3.reference(), false);
    assert_eq!(k3.encoding(), PublicKeyEncoding::Hex);
    assert_eq!(
        k3.data().as_str(),
        "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
    );

    let s = doc.to_string();
    assert_eq!(s.as_str(), flat);
}

#[test]
fn did_parse_document_3() {
    let jstr = r#"
    {
        "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
        "id": "did:example:123456789abcdefghi",
        "authentication": [
            "did:example:123456789abcdefghi#keys-1",
            "did:example:123456789abcdefghi#biometric-1",
            {
                "id": "did:example:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:example:123456789abcdefghi",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }
        ]
    }
    "#;

    let flat = r#"{"@context":["https://w3id.org/did/v1","https://w3id.org/security/v1"],"id":"did:example:123456789abcdefghi","authentication":["did:example:123456789abcdefghi#keys-1","did:example:123456789abcdefghi#biometric-1",{"id":"did:example:123456789abcdefghi#keys-2","type":"Ed25519VerificationKey2018","controller":"did:example:123456789abcdefghi","publicKeyBase58":"H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"}]}"#;

    let doc = Document::from_str(&jstr).unwrap();
    assert_eq!(doc.context().len(), 2);
    assert_eq!(doc.subject(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key().len(), 0);
    assert_eq!(doc.authentication().len(), 3);
    assert_eq!(doc.service().len(), 0);

    let k1 = &doc.authentication()[0];
    assert_eq!(k1.subject(), "did:example:123456789abcdefghi#keys-1");
    assert_eq!(k1.kind(), PublicKeyType::UnknownKey);
    assert_eq!(k1.controller(), "");
    assert_eq!(k1.encoding(), PublicKeyEncoding::Unknown);
    assert_eq!(k1.data(), "");
    assert!(k1.reference());

    let k2 = &doc.authentication()[1];
    assert_eq!(k2.subject(), "did:example:123456789abcdefghi#biometric-1");
    assert_eq!(k1.kind(), PublicKeyType::UnknownKey);
    assert_eq!(k1.controller(), "");
    assert_eq!(k1.encoding(), PublicKeyEncoding::Unknown);
    assert_eq!(k1.data(), "");
    assert!(k2.reference());

    let k3 = &doc.authentication()[2];
    assert_eq!(k3.subject(), "did:example:123456789abcdefghi#keys-2");
    assert_eq!(k3.kind(), PublicKeyType::Ed25519VerificationKey2018);
    assert_eq!(k3.controller(), "did:example:123456789abcdefghi");
    assert_eq!(k3.encoding(), PublicKeyEncoding::Base58);
    assert_eq!(k3.data(), "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV");
    assert_eq!(k3.reference(), false);

    let s = doc.to_string();
    assert_eq!(s.as_str(), flat);
}

#[test]
fn did_parse_document_4() {
    let jstr = r#"
    {
        "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
        "id": "did:example:123456789abcdefghi",
        "service": [{
            "id": "did:example:123456789abcdefghi#openid",
            "type": "OpenIdConnectVersion1.0Service",
            "serviceEndpoint": "https://openid.example.com/"
        }, {
            "id": "did:example:123456789abcdefghi#vcr",
            "type": "CredentialRepositoryService",
            "serviceEndpoint": "https://repository.example.com/service/8377464"
        }, {
            "id": "did:example:123456789abcdefghi#xdi",
            "type": "XdiService",
            "serviceEndpoint": "https://xdi.example.com/8377464"
        }, {
            "id": "did:example:123456789abcdefghi#agent",
            "type": "AgentService",
            "serviceEndpoint": "https://agent.example.com/8377464"
        }, {
            "@context": "did:example:contexts:987654321",
            "id": "did:example:123456789abcdefghi#hub",
            "type": "HubService",
            "serviceEndpoint": "https://hub.example.com/.identity/did:example:0123456789abcdef/"
        }, {
            "id": "did:example:123456789abcdefghi#messages",
            "type": "MessagingService",
            "serviceEndpoint": "https://example.com/messages/8377464"
        }, {
            "id": "did:example:123456789abcdefghi#inbox",
            "type": "SocialWebInboxService",
            "serviceEndpoint": "https://social.example.com/83hfh37dj",
            "description": "My public social inbox",
            "spamCost": {
                "amount": "0.50",
                "currency": "USD"
            }
        }, {
            "id": "did:example:123456789abcdefghi#authpush",
            "type": "DidAuthPushModeVersion1",
            "serviceEndpoint": "http://auth.example.com/did:example:123456789abcdefg"
        }]
    }
    "#;

    let flat = r#"{"@context":["https://w3id.org/did/v1","https://w3id.org/security/v1"],"id":"did:example:123456789abcdefghi","service":[{"id":"did:example:123456789abcdefghi#openid","type":"OpenIdConnectVersion1.0Service","serviceEndpoint":"https://openid.example.com/"},{"id":"did:example:123456789abcdefghi#vcr","type":"CredentialRepositoryService","serviceEndpoint":"https://repository.example.com/service/8377464"},{"id":"did:example:123456789abcdefghi#xdi","type":"XdiService","serviceEndpoint":"https://xdi.example.com/8377464"},{"id":"did:example:123456789abcdefghi#agent","type":"AgentService","serviceEndpoint":"https://agent.example.com/8377464"},{"@context":"did:example:contexts:987654321","id":"did:example:123456789abcdefghi#hub","type":"HubService","serviceEndpoint":"https://hub.example.com/.identity/did:example:0123456789abcdef/"},{"id":"did:example:123456789abcdefghi#messages","type":"MessagingService","serviceEndpoint":"https://example.com/messages/8377464"},{"id":"did:example:123456789abcdefghi#inbox","type":"SocialWebInboxService","serviceEndpoint":"https://social.example.com/83hfh37dj","description":"My public social inbox","spamCost":{"amount":"0.50","currency":"USD"}},{"id":"did:example:123456789abcdefghi#authpush","type":"DidAuthPushModeVersion1","serviceEndpoint":"http://auth.example.com/did:example:123456789abcdefg"}]}"#;

    let doc = Document::from_str(jstr).unwrap();
    assert_eq!(doc.context().len(), 2);
    assert_eq!(doc.subject(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key().len(), 0);
    assert_eq!(doc.authentication().len(), 0);
    assert_eq!(doc.service().len(), 8);

    let s1 = &doc.service()[0];
    assert!(s1.context().is_empty());
    assert_eq!(s1.subject(), "did:example:123456789abcdefghi#openid");
    assert_eq!(s1.kind(), "OpenIdConnectVersion1.0Service");
    assert_eq!(s1.endpoint(), "https://openid.example.com/");

    let s7 = &doc.service()[6];
    assert_eq!(s7.subject(), "did:example:123456789abcdefghi#inbox");
    assert_eq!(s7.kind(), "SocialWebInboxService");
    assert_eq!(s7.endpoint(), "https://social.example.com/83hfh37dj");
    assert_eq!(s7.extra["description"], "My public social inbox");
    assert_eq!(s7.extra["spamCost"]["amount"], "0.50");
    assert_eq!(s7.extra["spamCost"]["currency"], "USD");

    let s = doc.to_string();
    assert_eq!(s.as_str(), flat);
}

#[test]
fn did_parse_document_5() {
    let jstr = r#"
    {
        "@context": "https://example.org/example-method/v1",
        "id": "did:example:123456789abcdefghi",
        "service": [{
            "@context": "did:example:contexts:987654321",
            "id": "did:example:123456789abcdefghi#photos",
            "type": "PhotoStreamService",
            "serviceEndpoint": "https://example.org/photos/379283"
        }]
    }
    "#;

    let flat = r#"{"@context":"https://example.org/example-method/v1","id":"did:example:123456789abcdefghi","service":[{"@context":"did:example:contexts:987654321","id":"did:example:123456789abcdefghi#photos","type":"PhotoStreamService","serviceEndpoint":"https://example.org/photos/379283"}]}"#;

    let doc = Document::from_str(&jstr).unwrap();
    assert_eq!(doc.context().len(), 1);
    assert_eq!(doc.subject(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key().len(), 0);
    assert_eq!(doc.authentication().len(), 0);
    assert_eq!(doc.service().len(), 1);

    let s1 = &doc.service()[0];
    assert_eq!(s1.context().is_empty(), false);
    assert_eq!(s1.context().as_vec()[0], "did:example:contexts:987654321");
    assert_eq!(s1.subject(), "did:example:123456789abcdefghi#photos");
    assert_eq!(s1.kind(), "PhotoStreamService");
    assert_eq!(s1.endpoint(), "https://example.org/photos/379283");

    let s = doc.to_string();
    assert_eq!(s.as_str(), flat);
}

#[test]
fn did_parse_document_6() {
    let jstr = r#"
    {
        "@context": "https://w3id.org/did/v1",
        "id": "did:example:123456789abcdefghi",
        "authentication": [{
            "id": "did:example:123456789abcdefghi#keys-1",
            "type": "RsaVerificationKey2018",
            "controller": "did:example:123456789abcdefghi",
            "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"
        }],
        "service": [{
            "type": "VerifiableCredentialService",
            "serviceEndpoint": "https://example.com/vc/"
        }]
    }
    "#;

    let flat = r#"{"@context":"https://w3id.org/did/v1","id":"did:example:123456789abcdefghi","authentication":[{"id":"did:example:123456789abcdefghi#keys-1","type":"RsaVerificationKey2018","controller":"did:example:123456789abcdefghi","publicKeyPem":"-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"}],"service":[{"type":"VerifiableCredentialService","serviceEndpoint":"https://example.com/vc/"}]}"#;

    let doc = Document::from_str(jstr).unwrap();
    assert_eq!(doc.context().len(), 1);
    assert_eq!(doc.subject(), "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key().len(), 0);
    assert_eq!(doc.authentication().len(), 1);
    assert_eq!(doc.service().len(), 1);

    let a1 = &doc.authentication()[0];
    assert_eq!(a1.subject(), "did:example:123456789abcdefghi#keys-1");
    assert_eq!(a1.kind(), PublicKeyType::RsaVerificationKey2018);
    assert_eq!(a1.controller(), "did:example:123456789abcdefghi");
    assert_eq!(a1.encoding(), PublicKeyEncoding::Pem);
    assert_eq!(a1.data(), "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n");
    assert_eq!(a1.reference(), false);

    let s1 = &doc.service()[0];
    assert!(s1.context().is_empty());
    assert_eq!(s1.kind(), "VerifiableCredentialService");
    assert_eq!(s1.endpoint(), "https://example.com/vc/");

    let s = doc.to_string();
    assert_eq!(s.as_str(), flat);
}
