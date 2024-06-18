use std::str::FromStr;
use elliptic_curve::{CurveArithmetic, NonZeroScalar, PrimeField, ProjectivePoint, Scalar};
use elliptic_curve::group::Curve;
use elliptic_curve::ops::{MulByGenerator, Reduce};
use elliptic_curve::rand_core::CryptoRngCore;
use crate::auth_session::AuthSession;


pub struct Prover<T: CurveArithmetic> {
    private_key: NonZeroScalar<T>,
    curve: T,
    X : T::AffinePoint,
    auth_session: AuthSession<T>
}

pub fn init<T: CurveArithmetic>(curve: T, hex_password: &str) -> Prover<T> {
    let p_key_result = NonZeroScalar::<T>::from_str(hex_password);
    match p_key_result { 
        Ok(p_key) => {
            Prover{
                curve,
                private_key: p_key,
                X: ProjectivePoint::<T>::mul_by_generator(&p_key).to_affine(),
                auth_session: AuthSession{C: None, R: None, e: None, r:None}
            }
        },
        Err(e) => panic!("Unable to parse, err = {e}")
    }
    
}
impl<T: CurveArithmetic> Prover<T> {
    pub fn gen_R(&mut self, rng: &mut impl CryptoRngCore){
        let r = NonZeroScalar::<T>::random(rng);
        let R = ProjectivePoint::<T>::mul_by_generator(&r).to_affine();
        self.auth_session.R = Some(R);
    }

    pub fn gen_e(&mut self)
    where T: CurveArithmetic
    {

        let r_bytes = self.auth_session.r.unwrap().to_repr();
        let r_scalar = Scalar::<T>::reduce_bytes(&r_bytes);

        let c_bytes = self.auth_session.C.unwrap().to_repr();
        let c_scalar = Scalar::<T>::reduce_bytes(&c_bytes);

        let private_key_bytes = self.private_key.to_repr();
        let private_key_scalar = Scalar::<T>::reduce_bytes(&private_key_bytes);


        let e_scalar: <T as CurveArithmetic>::Scalar = r_scalar + (c_scalar*private_key_scalar);

        let e = NonZeroScalar::<T>::from_repr(e_scalar.to_repr()).unwrap();
        self.auth_session.e = Some(e);
    }
}