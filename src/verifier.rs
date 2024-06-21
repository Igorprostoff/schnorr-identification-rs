use std::ops::Mul;
use elliptic_curve::{CurveArithmetic, Group, NonZeroScalar, ProjectivePoint};
use elliptic_curve::group::Curve;
use elliptic_curve::ops::MulByGenerator;
use elliptic_curve::rand_core::CryptoRngCore;
use rand::RngCore;
use crate::prover::{Prover};
use crate::auth_session::AuthSession;

pub struct Verifier<T:  CurveArithmetic> {
    curve: T,

    pub auth_session: AuthSession<T>
}
pub fn init<T: CurveArithmetic>(curve: T) -> Verifier<T> {
            Verifier{
                curve,
                auth_session: AuthSession{C: None, R: None, e: None, r:None, X:None}
            }
}
impl<T: CurveArithmetic> Verifier<T> {
    pub fn gen_c(&mut self){
        let mut random = rand::thread_rng().next_u64();
        let C = NonZeroScalar::<T>::from_uint(random.into()).unwrap();
        self.auth_session.C = Some(C);
    }

    pub fn consume_X(&mut self, X: T::ProjectivePoint)
        where T: CurveArithmetic
    {
        self.auth_session.X = Some(X);
    }
    pub fn consume_R(&mut self, R: T::ProjectivePoint)
        where T: CurveArithmetic
    {
        self.auth_session.R = Some(R);
    }
    pub fn verify_e(&mut self, e: NonZeroScalar<T>) -> bool {
        let eG = ProjectivePoint::<T>::mul_by_generator(&e);
        let cX = self.auth_session.X.unwrap().mul(self.auth_session.C.unwrap().as_ref());
        eG == self.auth_session.R.unwrap() + cX

    }
}
