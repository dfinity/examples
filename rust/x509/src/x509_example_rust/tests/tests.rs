use candid::{decode_one, encode_one, Principal};
use openssl::{
    ec::{EcGroup, EcKey},
    hash::MessageDigest,
    nid::Nid,
    pkey::{PKey, PKeyRef, Private},
    x509::{X509Name, X509Req, X509},
};
use pocket_ic::{PocketIc, PocketIcBuilder, WasmResult};
use std::convert::TryFrom;
use x509_example_rust::{CaKeyInformation, KeyName, PemCertificateRequest, X509CertificateString};

const WASM_PATH: &str = "../../target/wasm32-unknown-unknown/release/x509_example_rust.wasm";

mod smoke {
    use super::*;

    #[test]
    fn should_instantiate_pic() {
        let _pic = PocketIcBuilder::new()
            .with_application_subnet()
            .with_ii_subnet()
            .with_fiduciary_subnet()
            .build();
    }

    #[test]
    fn should_instantiate_pic_and_canister_id() {
        let (_pic, _canister_id) =
            pic_and_canister_id(CaKeyInformation::Ed25519(KeyName::TestKey1));
    }

    #[test]
    fn should_fetch_root_ca_certificate() {
        let (pic, canister_id) = pic_and_canister_id(CaKeyInformation::Ed25519(KeyName::TestKey1));
        let _root_certificate = fetch_root_ca_certificate(&pic, canister_id);
    }
}

mod ed25519 {
    use super::*;

    #[test]
    fn root_ca_certificate_should_be_valid() {
        let (pic, canister_id) = pic_and_canister_id(CaKeyInformation::Ed25519(KeyName::TestKey1));

        let root_certificate = fetch_root_ca_certificate(&pic, canister_id);

        assert!(
            root_certificate
                .verify(
                    &root_certificate
                        .public_key()
                        .expect("failed to get public key")
                )
                .expect("errors occurred while verifying root certificate"),
            "failed to verify root certificate"
        );
    }

    #[test]
    fn child_certificate_should_be_valid() {
        let (pic, canister_id) = pic_and_canister_id(CaKeyInformation::Ed25519(KeyName::TestKey1));

        let root_certificate = fetch_root_ca_certificate(&pic, canister_id);

        for (key, digest_type) in generate_child_keys() {
            println!("Requesting child certificate with key: {key:?}");

            let req = generate_child_certificate_request(&key, digest_type);
            req.verify(&key)
                .expect("failed to verify child certificate request");
            let pem_req = req
                .to_pem()
                .expect("failed to convert child certificate request to PEM");

            let child_certificate = generate_child_certificate(pem_req, &pic, canister_id);

            assert!(
                child_certificate
                    .verify(
                        &root_certificate
                            .public_key()
                            .expect("failed to get public key")
                    )
                    .expect("errors occurred while verifying child certificate"),
                "failed to verify child certificate"
            );
        }
    }
}

mod ecdsa_secp256k1 {
    use super::*;

    #[test]
    fn root_ca_certificate_should_be_valid() {
        let (pic, canister_id) =
            pic_and_canister_id(CaKeyInformation::EcdsaSecp256k1(KeyName::TestKey1));

        let root_certificate = fetch_root_ca_certificate(&pic, canister_id);

        assert!(
            root_certificate
                .verify(
                    &root_certificate
                        .public_key()
                        .expect("failed to get public key")
                )
                .expect("errors occurred while verifying root certificate"),
            "failed to verify root certificate"
        );
    }

    #[test]
    fn child_certificate_should_be_valid() {
        let (pic, canister_id) =
            pic_and_canister_id(CaKeyInformation::EcdsaSecp256k1(KeyName::TestKey1));

        let root_certificate = fetch_root_ca_certificate(&pic, canister_id);

        for (key, digest_type) in generate_child_keys() {
            println!("Requesting child certificate with key: {key:?}");

            let req = generate_child_certificate_request(&key, digest_type);
            req.verify(&key)
                .expect("failed to verify child certificate request");
            let pem_req = req
                .to_pem()
                .expect("failed to convert child certificate request to PEM");

            let child_certificate = generate_child_certificate(pem_req, &pic, canister_id);

            assert!(
                child_certificate
                    .verify(
                        &root_certificate
                            .public_key()
                            .expect("failed to get public key")
                    )
                    .expect("errors occurred while verifying child certificate"),
                "failed to verify child certificate"
            );
        }
    }
}

fn pic_and_canister_id(ca_key_information: CaKeyInformation) -> (PocketIc, Principal) {
    let pic = PocketIcBuilder::new()
        .with_application_subnet()
        .with_ii_subnet()
        .with_fiduciary_subnet()
        .build();

    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm = std::fs::read(WASM_PATH).expect("Wasm file not found, run 'dfx build'.");

    pic.install_canister(
        canister_id,
        wasm,
        encode_one(ca_key_information).unwrap(),
        None,
    );

    (pic, canister_id)
}

fn fetch_root_ca_certificate(pic: &PocketIc, canister_id: Principal) -> X509 {
    let root_certificate_result: Result<X509CertificateString, String> = match pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            "root_ca_certificate",
            encode_one(()).unwrap(),
        )
        .expect("Failed to call counter canister")
    {
        WasmResult::Reply(r) => decode_one(&r).expect("failed to decode reply"),
        WasmResult::Reject(r) => panic!("Call failed: {:?}", r),
    };

    let root_certificate_pem = root_certificate_result.expect("failed to compute root certificate");

    X509::from_pem(root_certificate_pem.x509_certificate_string.as_bytes())
        .expect("failed to decode root certificate")
}

fn generate_child_certificate(pem_req: Vec<u8>, pic: &PocketIc, canister_id: Principal) -> X509 {
    let pem_certificate_request = String::from_utf8(pem_req).expect("invalid request encoding");
    let child_certificate_result: Result<X509CertificateString, String> = match pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            "child_certificate",
            encode_one(PemCertificateRequest {
                pem_certificate_request,
            })
            .unwrap(),
        )
        .expect("Failed to call counter canister")
    {
        WasmResult::Reply(r) => decode_one(&r).expect("failed to decode reply"),
        WasmResult::Reject(r) => panic!("Call failed: {:?}", r),
    };

    let child_certificate_text =
        child_certificate_result.expect("failed to compute child certificate");

    X509::from_pem(child_certificate_text.x509_certificate_string.as_bytes())
        .expect("failed to decode child certificate")
}

fn generate_child_certificate_request(
    key: &PKeyRef<Private>,
    digest_type: MessageDigest,
) -> X509Req {
    let mut builder = X509Name::builder().expect("failed to create X509NameBuilder");
    builder
        .append_entry_by_text("CN", "Test Corporation")
        .expect("failed to append entry");
    builder
        .append_entry_by_text("O", "Test Inc")
        .expect("failed to append entry");
    builder
        .append_entry_by_text("C", "US")
        .expect("failed to append entry");
    let subject_name = builder.build();

    let mut req_builder = X509Req::builder().expect("failed to create X509Req builder");

    req_builder
        .set_version(0)
        .expect("failed to set version in child certificate");

    req_builder
        .set_subject_name(&subject_name)
        .expect("failed to set subject name in child certificate");
    req_builder
        .set_pubkey(&key)
        .expect("failed to set public key in child certificate");
    req_builder
        .sign(&key, digest_type)
        .expect("failed to sign X509Req");

    req_builder.build()
}

fn generate_child_keys() -> Vec<(PKey<Private>, MessageDigest)> {
    let ed25519_key = PKey::generate_ed25519().expect("failed to generate key");

    let ec_group = EcGroup::from_curve_name(Nid::SECP256K1).expect("failed to create EC group");
    let ecdsa_key =
        PKey::from_ec_key(EcKey::generate(&ec_group).expect("failed to generate ECDSA key"))
            .unwrap();

    vec![
        (ed25519_key, MessageDigest::null()),
        (ecdsa_key, MessageDigest::sha256()),
    ]
}

#[test]
fn test_strum_produces_expected_key_names() {
    assert_eq!(
        <&'static str>::try_from(KeyName::DfxTestKey),
        Ok("dfx_test_key")
    );
    assert_eq!(
        <&'static str>::try_from(KeyName::TestKey1),
        Ok("test_key_1")
    );
    assert_eq!(<&'static str>::try_from(KeyName::Key1), Ok("key_1"));
}
