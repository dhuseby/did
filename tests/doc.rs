extern crate did;

use did::{Document, fields::PublicKeyType, fields::PublicKeyDataType};
use serde_json;

#[test]
fn did_parse_document_0() {
    let jstr = r#"
    {
        "@context": "https://w3id.org/did/v1",
        "id": "did:example:123456789abcdefghi"
    }
    "#;

    let flat = r#"{"@context":"https://w3id.org/did/v1","id":"did:example:123456789abcdefghi"}"#;

    let doc: Document = serde_json::from_str(jstr).unwrap();
    assert_eq!(doc.context.as_vec().len(), 1);
    assert_eq!(doc.id, "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key.len(), 0);
    //assert_eq!(doc.authentication.len(), 0);
    assert_eq!(doc.service.len(), 0);

    let s: String = serde_json::to_string(&doc).unwrap();
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

    let doc: Document = serde_json::from_str(jstr).unwrap();
    assert_eq!(doc.context.as_vec().len(), 2);
    assert_eq!(doc.id, "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key.len(), 0);
    //assert_eq!(doc.authentication.len(), 0);
    assert_eq!(doc.service.len(), 0);

    let s: String = serde_json::to_string(&doc).unwrap();
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

    let doc: Document = serde_json::from_str(jstr).unwrap();
    assert_eq!(doc.context.as_vec().len(), 2);
    assert_eq!(doc.id, "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key.len(), 3);
    //assert_eq!(doc.authentication.len(), 0);
    //assert_eq!(doc.service.len(), 0);

    assert_eq!(doc.public_key[0].id, "did:example:123456789abcdefghi#keys-1");
    assert_eq!(doc.public_key[0].key_type, PublicKeyType::RsaVerificationKey2018);
    assert_eq!(doc.public_key[0].controller, "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key[0].key_data.as_str(), "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----");

    assert_eq!(doc.public_key[1].id, "did:example:123456789abcdefghi#keys-2");
    assert_eq!(doc.public_key[1].key_type, PublicKeyType::Ed25519VerificationKey2018);
    assert_eq!(doc.public_key[1].controller, "did:example:pqrstuvwxyz0987654321");
    assert_eq!(doc.public_key[1].key_data.as_str(), "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV");

    assert_eq!(doc.public_key[2].id, "did:example:123456789abcdefghi#keys-3");
    assert_eq!(doc.public_key[2].key_type, PublicKeyType::EcdsaSecp256k1VerificationKey2019);
    assert_eq!(doc.public_key[2].controller, "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key[2].key_data.as_str(), "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71");

    let s: String = serde_json::to_string(&doc).unwrap();
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

    let doc: Document = serde_json::from_str(jstr).unwrap();
    assert_eq!(doc.context.as_vec().len(), 2);
    assert_eq!(doc.id, "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key.len(), 0);
    assert_eq!(doc.authentication.len(), 3);
    assert_eq!(doc.service.len(), 0);
    assert_eq!(doc.authentication[0].id, "did:example:123456789abcdefghi#keys-1");
    assert!(doc.authentication[0].reference);
    assert_eq!(doc.authentication[1].id, "did:example:123456789abcdefghi#biometric-1");
    assert!(doc.authentication[1].reference);
    assert_eq!(doc.authentication[2].id, "did:example:123456789abcdefghi#keys-2");
    assert_eq!(doc.authentication[2].key_type, PublicKeyType::Ed25519VerificationKey2018);
    assert_eq!(doc.authentication[2].controller, "did:example:123456789abcdefghi");
    assert_eq!(doc.authentication[2].key_data_type, PublicKeyDataType::Base58);
    assert_eq!(doc.authentication[2].key_data, "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV");
    assert_eq!(doc.authentication[2].reference, false);
    
    let s: String = serde_json::to_string(&doc).unwrap();
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

    let doc: Document = serde_json::from_str(jstr).unwrap();
    assert_eq!(doc.context.as_vec().len(), 2);
    assert_eq!(doc.id, "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key.len(), 0);
    //assert_eq!(doc.authentication.len(), 0);
    assert_eq!(doc.service.len(), 8);

    assert!(doc.service[0].context.is_missing());
    assert_eq!(doc.service[0].id, "did:example:123456789abcdefghi#openid");
    assert_eq!(doc.service[0].service_type, "OpenIdConnectVersion1.0Service");
    assert_eq!(doc.service[0].endpoint, "https://openid.example.com/");

    assert_eq!(doc.service[6].extra["description"], "My public social inbox");
    assert_eq!(doc.service[6].extra["spamCost"]["amount"], "0.50");
    assert_eq!(doc.service[6].extra["spamCost"]["currency"], "USD");

    let s: String = serde_json::to_string(&doc).unwrap();
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

    let doc: Document = serde_json::from_str(jstr).unwrap();
    assert_eq!(doc.context.as_vec().len(), 1);
    assert_eq!(doc.id, "did:example:123456789abcdefghi");
    assert_eq!(doc.public_key.len(), 0);
    //assert_eq!(doc.authentication.len(), 0);
    assert_eq!(doc.service.len(), 1);

    assert_eq!(doc.service[0].context.is_missing(), false);
    assert_eq!(doc.service[0].context.as_vec()[0], "did:example:contexts:987654321");
    assert_eq!(doc.service[0].id, "did:example:123456789abcdefghi#photos");
    assert_eq!(doc.service[0].service_type, "PhotoStreamService");
    assert_eq!(doc.service[0].endpoint, "https://example.org/photos/379283");

    let s: String = serde_json::to_string(&doc).unwrap();
    assert_eq!(s.as_str(), flat);
}
