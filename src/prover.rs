use std::str::FromStr;
use elliptic_curve::{CurveArithmetic, FieldBytes, NonZeroScalar, PrimeField, ProjectivePoint, Scalar};
use elliptic_curve::generic_array::arr;
use elliptic_curve::group::Curve;
use elliptic_curve::ops::{MulByGenerator, Reduce};
use elliptic_curve::point::AffineCoordinates;
use elliptic_curve::rand_core::CryptoRngCore;
use rand::RngCore;
use crate::auth_session::AuthSession;
use base64::{Engine, engine::general_purpose::STANDARD, read::DecoderReader};

pub struct Prover<T: CurveArithmetic> {
    private_key: NonZeroScalar<T>,
    curve: T,
    pub auth_session: AuthSession<T>
}

pub fn init<T: CurveArithmetic>(curve: T, hex_password: &str) -> Prover<T> {
    let p_key_result = NonZeroScalar::<T>::from_str(hex_password);
    match p_key_result { 
        Ok(p_key) => {
            Prover{
                curve,
                private_key: p_key,
                auth_session: AuthSession{C: None, R: None, e: None, r:None, X: Some(ProjectivePoint::<T>::mul_by_generator(&p_key))}
            }
        },
        Err(e) => panic!("Unable to parse, err = {e}")
    }
    
}
impl<T: CurveArithmetic> Prover<T> {
    pub fn gen_R(&mut self){
        let mut random = rand::thread_rng().next_u64();
        let r = NonZeroScalar::<T>::from_uint(random.into()).unwrap();
        let R = ProjectivePoint::<T>::mul_by_generator(&r);
        self.auth_session.R = Some(R);
        self.auth_session.r = Some(r);
    }
    
    pub fn serialize_R(&mut self) -> Option<String>{
        let mut R_x_coord: FieldBytes<T>= FieldBytes::<T>::default();

        match self.auth_session.R {
            Some(R) => {
                R_x_coord = R.to_affine().x()
                
            },
            None => {
                return None
            }
        }
        Some(STANDARD.encode(R_x_coord))
        
    }

    pub fn consume_c(&mut self, c: NonZeroScalar<T>)
        where T: CurveArithmetic
    {
        self.auth_session.C = Some(c);
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