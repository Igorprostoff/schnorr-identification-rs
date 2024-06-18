use elliptic_curve::{CurveArithmetic, NonZeroScalar};
use elliptic_curve::rand_core::CryptoRngCore;
use crate::prover::{Prover};
use crate::auth_session::AuthSession;

pub struct Verifier<T:  CurveArithmetic> {
    curve: T,
    X : T::AffinePoint,
    auth_session: AuthSession<T>
}

impl<T: CurveArithmetic> Verifier<T> {
    pub fn gen_C(&mut self, rng: &mut impl CryptoRngCore){
        let C = NonZeroScalar::<T>::random(rng);
        self.auth_session.C = Some(C);
    }
}
