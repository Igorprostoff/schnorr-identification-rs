use ark_ec::AffineRepr;
use ark_ec::hashing::HashToCurve;
use ark_test_curves::secp256k1::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField, Config, FrConfig};
use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ff::{Fp256, MontBackend};

pub struct AuthSession {
    pub R : Option<Affine<Config>>,
    pub C: Option<Fp256<MontBackend<FrConfig, 4>>>,
    pub e: Option<Fp256<MontBackend<FrConfig, 4>>>,
    pub r: Option<Fp256<MontBackend<FrConfig, 4>>>, //Only available for prover
    pub X : Option<Affine<Config>>,
}