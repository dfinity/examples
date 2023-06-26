#![allow(unused)]

pub use ic_crypto_internal_bls12_381_type::*;
use rand::{CryptoRng, RngCore};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub type NodeIndex = u32;

#[derive(Copy, Clone, Debug)]
pub enum TransportSecretKeyDeserializationError {
    InvalidSecretKey,
}

#[derive(Clone)]
pub struct DerivationPath {
    delta: Scalar,
}

/// Decode a scalar as a big-endian byte string, reducing modulo group order
pub fn scalar_from_wide_bytes(input: &[u8; 64]) -> Scalar {
    let mut le_bytes = *input;
    le_bytes.reverse();
    let s = ic_bls12_381::Scalar::from_bytes_wide(&le_bytes);
    le_bytes.zeroize();

    let mut bytes = s.to_bytes();
    bytes.reverse();
    Scalar::deserialize_unchecked(&bytes)
}

impl DerivationPath {
    pub fn new2<U: AsRef<[u8]>>(canister_id: &[u8], extra_paths: &[U]) -> Self {
        let mut shake256 = Shake256::default();
        shake256.update(b"ic-crypto-vetkd-bls12-381-derivation-path\0");

        shake256.update((canister_id.len() as u64).to_be_bytes());
        shake256.update(canister_id);

        for path in extra_paths {
            shake256.update((path.as_ref().len() as u64).to_be_bytes());
            shake256.update(path.as_ref());
        }

        let mut output = shake256.finalize_xof();

        let mut delta_h = [0u8; 64];
        output.read(&mut delta_h);
        let delta = scalar_from_wide_bytes(&delta_h);
        Self { delta }
    }

    pub fn new3(canister_id: &[u8], extra_paths: &[Vec<u8>]) -> Self {
        let mut shake256 = Shake256::default();
        shake256.update(b"ic-crypto-vetkd-bls12-381-derivation-path\0");

        shake256.update((canister_id.len() as u64).to_be_bytes());
        shake256.update(canister_id);

        for path in extra_paths {
            shake256.update((path.len() as u64).to_be_bytes());
            shake256.update(path);
        }

        let mut output = shake256.finalize_xof();

        let mut delta_h = [0u8; 64];
        output.read(&mut delta_h);
        let delta = scalar_from_wide_bytes(&delta_h);
        Self { delta }
    }

    pub fn new(canister_id: &[u8], extra_paths: &[&[u8]]) -> Self {
        let mut shake256 = Shake256::default();
        shake256.update(b"ic-crypto-vetkd-bls12-381-derivation-path\0");

        shake256.update((canister_id.len() as u64).to_be_bytes());
        shake256.update(canister_id);

        for path in extra_paths {
            shake256.update((path.len() as u64).to_be_bytes());
            shake256.update(path);
        }

        let mut output = shake256.finalize_xof();

        let mut delta_h = [0u8; 64];
        output.read(&mut delta_h);
        let delta = scalar_from_wide_bytes(&delta_h);
        Self { delta }
    }

    fn delta(&self) -> &Scalar {
        &self.delta
    }
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct TransportSecretKey {
    secret_key: Scalar,
}

impl TransportSecretKey {
    pub fn generate<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        let secret_key = Scalar::random(rng);
        Self { secret_key }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.secret_key.serialize().to_vec()
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, TransportSecretKeyDeserializationError> {
        let secret_key = Scalar::deserialize(&bytes)
            .map_err(|_| TransportSecretKeyDeserializationError::InvalidSecretKey)?;
        Ok(Self { secret_key })
    }

    pub fn public_key(&self) -> TransportPublicKey {
        let public_key = G1Affine::generator() * &self.secret_key;
        TransportPublicKey::new(public_key.to_affine())
    }

    fn secret(&self) -> &Scalar {
        &self.secret_key
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TransportPublicKeyDeserializationError {
    InvalidPublicKey,
}

#[derive(Clone, Debug)]
pub struct TransportPublicKey {
    public_key: G1Affine,
}

impl TransportPublicKey {
    fn new(public_key: G1Affine) -> Self {
        Self { public_key }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.public_key.serialize().to_vec()
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, TransportPublicKeyDeserializationError> {
        let public_key = G1Affine::deserialize(&bytes)
            .map_err(|_| TransportPublicKeyDeserializationError::InvalidPublicKey)?;
        Ok(Self { public_key })
    }

    fn point(&self) -> &G1Affine {
        &self.public_key
    }
}

pub fn get_derived_public_key(pk: &G2Affine, derivation_path: &DerivationPath) -> G2Affine {
    G2Affine::from(G2Affine::generator() * derivation_path.delta() + pk)
}

fn augmented_hash_to_g1(pk: &G2Affine, data: &[u8]) -> G1Affine {
    let domain_sep = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_AUG_";

    let mut signature_input = vec![];
    signature_input.extend_from_slice(&pk.serialize());
    signature_input.extend_from_slice(data);
    G1Affine::hash(domain_sep, &signature_input)
}

fn check_validity(
    c1: &G1Affine,
    c2: &G2Affine,
    c3: &G1Affine,
    did: &[u8],
    tpk: &TransportPublicKey,
    master_pk: &G2Affine,
    signing_pk: &G2Affine,
) -> bool {
    let neg_g2_g = G2Prepared::neg_generator();
    let c2_prepared = G2Prepared::from(c2);

    // check e(c1,g2) == e(g1, c2)
    let c1_c2 = Gt::multipairing(&[(c1, neg_g2_g), (G1Affine::generator(), &c2_prepared)]);
    if !c1_c2.is_identity() {
        return false;
    }

    let msg = augmented_hash_to_g1(master_pk, did);
    let sig_key_prepared = G2Prepared::from(signing_pk);

    // check e(c3, g2) == e(tpk, c2) * e(msg, dpki)
    let c3_c2_msg = Gt::multipairing(&[
        (c3, neg_g2_g),
        (tpk.point(), &c2_prepared),
        (&msg, &sig_key_prepared),
    ]);

    if !c3_c2_msg.is_identity() {
        return false;
    }

    true
}

#[derive(Copy, Clone, Debug)]
pub enum EncryptedKeyDeserializationError {
    InvalidEncryptedKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncryptedKey {
    c1: G1Affine,
    c2: G2Affine,
    c3: G1Affine,
}

impl EncryptedKey {
    pub const BYTES: usize = 2 * G1Affine::BYTES + G2Affine::BYTES;

    pub fn combine(
        nodes: &[(NodeIndex, G2Affine, EncryptedKeyShare)],
        master_pk: &G2Affine,
        tpk: &TransportPublicKey,
        derivation_path: &DerivationPath,
        did: &[u8],
    ) -> Result<Self, Vec<NodeIndex>> {
        let l =
            LagrangeCoefficients::at_zero(&nodes.iter().map(|i| i.0).collect::<Vec<_>>()).unwrap();

        let c1 = l
            .interpolate_g1(&nodes.iter().map(|i| i.2.c1.clone()).collect::<Vec<_>>())
            .unwrap();
        let c2 = l
            .interpolate_g2(&nodes.iter().map(|i| i.2.c2.clone()).collect::<Vec<_>>())
            .unwrap();
        let c3 = l
            .interpolate_g1(&nodes.iter().map(|i| i.2.c3.clone()).collect::<Vec<_>>())
            .unwrap();

        let dpk = get_derived_public_key(master_pk, derivation_path);

        if !check_validity(&c1, &c2, &c3, did, tpk, &dpk, &dpk) {
            let mut invalid = vec![];

            for (node_id, node_pk, node_eks) in nodes {
                if !node_eks.is_valid(master_pk, node_pk, derivation_path, did, tpk) {
                    invalid.push(*node_id);
                }
            }

            return Err(invalid);
        }

        Ok(Self { c1, c2, c3 })
    }

    pub fn decrypt(
        &self,
        tsk: &TransportSecretKey,
        master_pk: &G2Affine,
        derivation_path: &DerivationPath,
        did: &[u8],
    ) -> Option<G1Affine> {
        let dpk = get_derived_public_key(master_pk, derivation_path);
        let msg = augmented_hash_to_g1(&dpk, did);

        let k = G1Affine::from(G1Projective::from(&self.c3) - &self.c1 * tsk.secret());

        let dpk_prep = G2Prepared::from(dpk);
        let k_is_valid_sig =
            Gt::multipairing(&[(&k, G2Prepared::neg_generator()), (&msg, &dpk_prep)]).is_identity();

        if k_is_valid_sig {
            Some(k)
        } else {
            None
        }
    }

    pub fn deserialize(val: [u8; Self::BYTES]) -> Result<Self, EncryptedKeyDeserializationError> {
        let c2_start = G1Affine::BYTES;
        let c3_start = G1Affine::BYTES + G2Affine::BYTES;

        let c1_bytes: &[u8] = &val[..c2_start];
        let c2_bytes: &[u8] = &val[c2_start..c3_start];
        let c3_bytes: &[u8] = &val[c3_start..];

        let c1 = G1Affine::deserialize(&c1_bytes);
        let c2 = G2Affine::deserialize(&c2_bytes);
        let c3 = G1Affine::deserialize(&c3_bytes);

        match (c1, c2, c3) {
            (Ok(c1), Ok(c2), Ok(c3)) => Ok(Self { c1, c2, c3 }),
            (_, _, _) => Err(EncryptedKeyDeserializationError::InvalidEncryptedKey),
        }
    }

    pub fn serialize(&self) -> [u8; Self::BYTES] {
        let mut output = [0u8; Self::BYTES];

        let c2_start = G1Affine::BYTES;
        let c3_start = G1Affine::BYTES + G2Affine::BYTES;

        output[..c2_start].copy_from_slice(&self.c1.serialize());
        output[c2_start..c3_start].copy_from_slice(&self.c2.serialize());
        output[c3_start..].copy_from_slice(&self.c3.serialize());

        output
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncryptedKeyShare {
    c1: G1Affine,
    c2: G2Affine,
    c3: G1Affine,
}

#[derive(Copy, Clone, Debug)]
pub enum EncryptedKeyShareDeserializationError {
    InvalidEncryptedKeyShare,
}

impl EncryptedKeyShare {
    pub const BYTES: usize = 2 * G1Affine::BYTES + G2Affine::BYTES;

    pub fn create<R: RngCore + CryptoRng>(
        rng: &mut R,
        master_pk: &G2Affine,
        node_sk: &Scalar,
        transport_pk: &TransportPublicKey,
        derivation_path: &DerivationPath,
        did: &[u8],
    ) -> Self {
        let delta = derivation_path.delta();

        let dsk = delta + node_sk;
        let dpk = G2Affine::from(G2Affine::generator() * delta + master_pk);

        let r = Scalar::random(rng);

        let msg = augmented_hash_to_g1(&dpk, did);

        let c1 = G1Affine::from(G1Affine::generator() * &r);
        let c2 = G2Affine::from(G2Affine::generator() * &r);
        let c3 = G1Affine::from(transport_pk.point() * &r + msg * &dsk);

        Self { c1, c2, c3 }
    }

    pub fn is_valid(
        &self,
        pk: &G2Affine,
        pki: &G2Affine,
        derivation_path: &DerivationPath,
        did: &[u8],
        tpk: &TransportPublicKey,
    ) -> bool {
        let dpki = get_derived_public_key(pki, derivation_path);
        let dpk = get_derived_public_key(pk, derivation_path);

        check_validity(&self.c1, &self.c2, &self.c3, did, tpk, &dpk, &dpki)
    }

    pub fn deserialize(
        val: [u8; Self::BYTES],
    ) -> Result<Self, EncryptedKeyShareDeserializationError> {
        let c2_start = G1Affine::BYTES;
        let c3_start = G1Affine::BYTES + G2Affine::BYTES;

        let c1_bytes: &[u8] = &val[..c2_start];
        let c2_bytes: &[u8] = &val[c2_start..c3_start];
        let c3_bytes: &[u8] = &val[c3_start..];

        let c1 = G1Affine::deserialize(&c1_bytes);
        let c2 = G2Affine::deserialize(&c2_bytes);
        let c3 = G1Affine::deserialize(&c3_bytes);

        match (c1, c2, c3) {
            (Ok(c1), Ok(c2), Ok(c3)) => Ok(Self { c1, c2, c3 }),
            (_, _, _) => Err(EncryptedKeyShareDeserializationError::InvalidEncryptedKeyShare),
        }
    }

    pub fn serialize(&self) -> [u8; Self::BYTES] {
        let mut output = [0u8; Self::BYTES];

        let c2_start = G1Affine::BYTES;
        let c3_start = G1Affine::BYTES + G2Affine::BYTES;

        output[..c2_start].copy_from_slice(&self.c1.serialize());
        output[c2_start..c3_start].copy_from_slice(&self.c2.serialize());
        output[c3_start..].copy_from_slice(&self.c3.serialize());

        output
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LagrangeError {
    InterpolationError,
}

pub struct LagrangeCoefficients {
    coefficients: Vec<Scalar>,
}

impl LagrangeCoefficients {
    fn new(coefficients: Vec<Scalar>) -> Result<Self, LagrangeError> {
        if coefficients.is_empty() {
            return Err(LagrangeError::InterpolationError);
        }

        Ok(Self { coefficients })
    }

    pub fn coefficients(&self) -> &[Scalar] {
        &self.coefficients
    }

    /// Given a list of samples `(x, f(x) * g)` for a set of unique `x`, some
    /// polynomial `f`, and some elliptic curve point `g`, returns `f(value) * g`.
    ///
    /// See: <https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#Computationally_efficient_approach>
    pub fn interpolate_g1(&self, y: &[G1Affine]) -> Result<G1Affine, LagrangeError> {
        if y.len() != self.coefficients.len() {
            return Err(LagrangeError::InterpolationError);
        }

        Ok(G1Projective::muln_affine_vartime(y, &self.coefficients).to_affine())
    }

    /// Given a list of samples `(x, f(x) * g)` for a set of unique `x`, some
    /// polynomial `f`, and some elliptic curve point `g`, returns `f(value) * g`.
    ///
    /// See: <https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#Computationally_efficient_approach>
    pub fn interpolate_g2(&self, y: &[G2Affine]) -> Result<G2Affine, LagrangeError> {
        if y.len() != self.coefficients.len() {
            return Err(LagrangeError::InterpolationError);
        }

        Ok(G2Projective::muln_affine_vartime(y, &self.coefficients).to_affine())
    }

    /// Given a list of samples `(x, f(x))` for a set of unique `x`, some
    /// polynomial `f`, returns `f(value) * g`.
    ///
    /// See: <https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#Computationally_efficient_approach>
    pub fn interpolate_scalar(&self, y: &[Scalar]) -> Result<Scalar, LagrangeError> {
        if y.len() != self.coefficients.len() {
            return Err(LagrangeError::InterpolationError);
        }

        Ok(Scalar::muln_vartime(y, &self.coefficients))
    }

    /// Check for duplicate dealer indexes
    ///
    /// Since these are public we don't need to worry about the lack of constant
    /// time behavior from HashSet
    fn check_for_duplicates(node_index: &[NodeIndex]) -> Result<(), LagrangeError> {
        let mut set = std::collections::HashSet::new();

        for i in node_index {
            if !set.insert(i) {
                return Err(LagrangeError::InterpolationError);
            }
        }

        Ok(())
    }

    /// Computes Lagrange polynomials evaluated at zero
    ///
    /// Namely it computes the following values:
    ///    * lagrange_i = numerator_i/denominator_i
    ///    * numerator_i = (x_0) * (x_1) * ... * (x_(i-1)) *(x_(i+1)) * ... *(x_n)
    ///    * denominator_i = (x_0 - x_i) * (x_1 - x_i) * ... * (x_(i-1) - x_i) *
    ///      (x_(i+1) - x_i) * ... * (x_n - x_i)
    pub fn at_zero(samples: &[NodeIndex]) -> Result<Self, LagrangeError> {
        Self::at_value(&Scalar::zero(), samples)
    }

    /// Computes Lagrange polynomials evaluated at a given value.
    ///
    /// Namely it computes the following values:
    ///    * lagrange_i = numerator_i/denominator_i
    ///    * numerator_i = (x_0-value) * (x_1-value) * ... * (x_(i-1)-value) *(x_(i+1)-value) * ... *(x_n-value)
    ///    * denominator_i = (x_0 - x_i) * (x_1 - x_i) * ... * (x_(i-1) - x_i) *
    ///      (x_(i+1) - x_i) * ... * (x_n - x_i)
    pub fn at_value(value: &Scalar, samples: &[NodeIndex]) -> Result<Self, LagrangeError> {
        // This is not strictly required but for our usage it simplifies matters
        if samples.is_empty() {
            return Err(LagrangeError::InterpolationError);
        }

        if samples.len() == 1 {
            return Self::new(vec![Scalar::one()]);
        }

        Self::check_for_duplicates(samples)?;

        let samples: Vec<Scalar> = samples.iter().map(|&s| Scalar::from_u32(s + 1)).collect();

        let mut numerator = Vec::with_capacity(samples.len());
        let mut tmp = Scalar::one();
        numerator.push(tmp.clone());
        for x in samples.iter().take(samples.len() - 1) {
            tmp *= x - value;
            numerator.push(tmp.clone());
        }

        tmp = Scalar::one();
        for (i, x) in samples[1..].iter().enumerate().rev() {
            tmp *= x - value;
            numerator[i] *= &tmp;
        }

        for (lagrange_i, x_i) in numerator.iter_mut().zip(&samples) {
            // Compute the value at 0 of the i-th Lagrange polynomial that is `0` at the
            // other data points but `1` at `x_i`.
            let mut denom = Scalar::one();
            for x_j in samples.iter().filter(|x_j| *x_j != x_i) {
                denom *= x_j - x_i;
            }

            let inv = match denom.inverse() {
                None => return Err(LagrangeError::InterpolationError),
                Some(inv) => inv,
            };

            *lagrange_i *= inv;
        }
        Self::new(numerator)
    }
}
