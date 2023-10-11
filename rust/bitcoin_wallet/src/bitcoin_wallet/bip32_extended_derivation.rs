pub(crate) fn extended_bip32_derivation(
    public_key: &[u8],
    chain_code: &[u8],
    path: &[Vec<u8>],
) -> (Vec<u8>, Vec<u8>) {
    fn secp256k1_decode_point(bytes: &[u8]) -> Option<k256::ProjectivePoint> {
        use k256::elliptic_curve::sec1::FromEncodedPoint;

        match k256::EncodedPoint::from_bytes(bytes) {
            Ok(ept) => {
                let apt = k256::AffinePoint::from_encoded_point(&ept);

                if bool::from(apt.is_some()) {
                    Some(k256::ProjectivePoint::from(apt.unwrap()))
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn secp256k1_decode_scalar(bytes: &[u8]) -> Option<k256::Scalar> {
        use k256::elliptic_curve::group::ff::PrimeField;

        if bytes.len() != 32 {
            return None;
        }

        let fb = k256::FieldBytes::from_slice(bytes);
        let s = k256::Scalar::from_repr(*fb);

        if bool::from(s.is_some()) {
            Some(s.unwrap())
        } else {
            None
        }
    }

    fn secp256k1_add(public_key: &[u8], offset: &[u8]) -> Vec<u8> {
        use k256::elliptic_curve::group::GroupEncoding;

        let scalar = secp256k1_decode_scalar(offset).expect("Invalid scalar");

        let public_key = secp256k1_decode_point(public_key).expect("Invalid public key");

        let g = k256::AffinePoint::GENERATOR;
        let g_o = g * scalar;

        let pk_p_g_o = public_key + g_o;

        pk_p_g_o.to_affine().to_bytes().to_vec()
    }

    fn ckdpub(public_key: &[u8], chain_code: &[u8], index: &[u8]) -> (Vec<u8>, Vec<u8>) {
        use hmac::{Hmac, Mac};
        use sha2::Sha512;

        let mut hmac =
            Hmac::<Sha512>::new_from_slice(chain_code).expect("HMAC unable to accept chain code");
        hmac.update(public_key);
        hmac.update(index);
        let hmac_output = hmac.finalize().into_bytes();

        let new_public_key = secp256k1_add(public_key, &hmac_output[..32]);
        let new_chain_code = hmac_output[32..].to_vec();
        (new_public_key, new_chain_code)
    }

    let mut public_key = public_key.to_vec();
    let mut chain_code = if chain_code.is_empty() {
        vec![0; 32]
    } else {
        chain_code.to_vec()
    };

    for idx in path {
        let (new_public_key, new_chain_code) = ckdpub(&public_key, &chain_code, idx);

        public_key = new_public_key;
        chain_code = new_chain_code;
    }

    (public_key, chain_code)
}
