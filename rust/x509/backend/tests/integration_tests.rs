use candid::{decode_one, encode_one, Principal};
use openssl::{
    ec::{EcGroup, EcKey},
    hash::MessageDigest,
    nid::Nid,
    pkey::{PKey, PKeyRef, Private},
    x509::{X509Name, X509Req, X509},
};
use pocket_ic::PocketIcBuilder;
use backend::{CaKeyInformation, PemCertificateRequest, X509CertificateString};

fn load_wasm() -> Vec<u8> {
    let wasm_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../target/wasm32-unknown-unknown/release/backend.wasm");
    std::fs::read(&wasm_path).unwrap_or_else(|_| panic!(
        "WASM not found at {}. Run: icp build backend",
        wasm_path.display()
    ))
}

fn pic_and_canister_id(ca_key_information: CaKeyInformation) -> (pocket_ic::PocketIc, Principal) {
    // with_test_threshold_keys_subnet provides test_key_1 and dfx_test_key for all algorithms.
    let pic = PocketIcBuilder::new()
        .with_application_subnet()
        .with_test_threshold_keys_subnet()
        .build();

    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    pic.install_canister(
        canister_id,
        load_wasm(),
        encode_one(ca_key_information).unwrap(),
        None,
    );

    (pic, canister_id)
}

fn fetch_root_ca_certificate(pic: &pocket_ic::PocketIc, canister_id: Principal) -> X509 {
    let reply = pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            "root_ca_certificate",
            encode_one(()).unwrap(),
        )
        .expect("Failed to call root_ca_certificate");

    let result: Result<X509CertificateString, String> =
        decode_one(&reply).expect("failed to decode reply");
    let pem = result.expect("failed to compute root certificate");

    X509::from_pem(pem.x509_certificate_string.as_bytes())
        .expect("failed to decode root certificate PEM")
}

fn generate_child_certificate(
    pem_req: Vec<u8>,
    pic: &pocket_ic::PocketIc,
    canister_id: Principal,
) -> X509 {
    let pem_certificate_request =
        String::from_utf8(pem_req).expect("invalid request encoding");

    let reply = pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            "child_certificate",
            encode_one(PemCertificateRequest { pem_certificate_request }).unwrap(),
        )
        .expect("Failed to call child_certificate");

    let result: Result<X509CertificateString, String> =
        decode_one(&reply).expect("failed to decode reply");
    let pem = result.expect("failed to compute child certificate");

    X509::from_pem(pem.x509_certificate_string.as_bytes())
        .expect("failed to decode child certificate PEM")
}

fn generate_child_certificate_request(
    key: &PKeyRef<Private>,
    digest_type: MessageDigest,
) -> X509Req {
    let mut builder = X509Name::builder().expect("failed to create X509NameBuilder");
    builder.append_entry_by_text("CN", "Test Corporation").unwrap();
    builder.append_entry_by_text("O", "Test Inc").unwrap();
    builder.append_entry_by_text("C", "US").unwrap();
    let subject_name = builder.build();

    let mut req_builder = X509Req::builder().expect("failed to create X509Req builder");
    req_builder.set_version(0).unwrap();
    req_builder.set_subject_name(&subject_name).unwrap();
    req_builder.set_pubkey(key).unwrap();
    req_builder.sign(key, digest_type).unwrap();
    req_builder.build()
}

fn generate_child_keys() -> Vec<(PKey<Private>, MessageDigest)> {
    let ed25519_key = PKey::generate_ed25519().expect("failed to generate Ed25519 key");

    let ec_group = EcGroup::from_curve_name(Nid::SECP256K1).expect("failed to create EC group");
    let ecdsa_key = PKey::from_ec_key(
        EcKey::generate(&ec_group).expect("failed to generate ECDSA key"),
    )
    .unwrap();

    vec![
        (ed25519_key, MessageDigest::null()),
        (ecdsa_key, MessageDigest::sha256()),
    ]
}

mod smoke {
    use super::*;

    #[test]
    fn should_instantiate_pic() {
        let _pic = PocketIcBuilder::new()
            .with_application_subnet()
            .with_test_threshold_keys_subnet()
            .build();
    }

    #[test]
    fn should_instantiate_pic_and_canister_id() {
        let (_pic, _canister_id) =
            pic_and_canister_id(CaKeyInformation::Ed25519("test_key_1".to_string()));
    }

    #[test]
    fn should_fetch_root_ca_certificate() {
        let (pic, canister_id) =
            pic_and_canister_id(CaKeyInformation::Ed25519("test_key_1".to_string()));
        let _root_certificate = fetch_root_ca_certificate(&pic, canister_id);
    }
}

mod ed25519 {
    use super::*;

    #[test]
    fn root_ca_certificate_should_be_valid() {
        let (pic, canister_id) =
            pic_and_canister_id(CaKeyInformation::Ed25519("test_key_1".to_string()));
        let root_certificate = fetch_root_ca_certificate(&pic, canister_id);

        assert!(
            root_certificate
                .verify(&root_certificate.public_key().expect("failed to get public key"))
                .expect("errors while verifying root certificate"),
            "failed to verify root certificate"
        );
    }

    #[test]
    fn child_certificate_should_be_valid() {
        let (pic, canister_id) =
            pic_and_canister_id(CaKeyInformation::Ed25519("test_key_1".to_string()));
        let root_certificate = fetch_root_ca_certificate(&pic, canister_id);

        for (key, digest_type) in generate_child_keys() {
            let req = generate_child_certificate_request(&key, digest_type);
            req.verify(&key).expect("failed to verify CSR");
            let pem_req = req.to_pem().expect("failed to convert CSR to PEM");

            let child_certificate = generate_child_certificate(pem_req, &pic, canister_id);

            assert!(
                child_certificate
                    .verify(&root_certificate.public_key().unwrap())
                    .expect("errors while verifying child certificate"),
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
            pic_and_canister_id(CaKeyInformation::EcdsaSecp256k1("test_key_1".to_string()));
        let root_certificate = fetch_root_ca_certificate(&pic, canister_id);

        assert!(
            root_certificate
                .verify(&root_certificate.public_key().expect("failed to get public key"))
                .expect("errors while verifying root certificate"),
            "failed to verify root certificate"
        );
    }

    #[test]
    fn child_certificate_should_be_valid() {
        let (pic, canister_id) =
            pic_and_canister_id(CaKeyInformation::EcdsaSecp256k1("test_key_1".to_string()));
        let root_certificate = fetch_root_ca_certificate(&pic, canister_id);

        for (key, digest_type) in generate_child_keys() {
            let req = generate_child_certificate_request(&key, digest_type);
            req.verify(&key).expect("failed to verify CSR");
            let pem_req = req.to_pem().expect("failed to convert CSR to PEM");

            let child_certificate = generate_child_certificate(pem_req, &pic, canister_id);

            assert!(
                child_certificate
                    .verify(&root_certificate.public_key().unwrap())
                    .expect("errors while verifying child certificate"),
                "failed to verify child certificate"
            );
        }
    }
}
