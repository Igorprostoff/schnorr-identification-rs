use elliptic_curve::{CurveArithmetic, NonZeroScalar};

pub struct AuthSession<T: CurveArithmetic> {
    pub R : Option<T::AffinePoint>,
    pub C: Option<NonZeroScalar<T>>,
    pub e: Option<NonZeroScalar<T>>,
    pub r: Option<NonZeroScalar<T>> //Only available for prover
}