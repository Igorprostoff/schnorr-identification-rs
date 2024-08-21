use alloc::string::{String, ToString};
use core::ops::Mul;
use core::str::FromStr;
use ark_ec::{CurveGroup, PrimeGroup};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use crate::auth_session::AuthSession;
use ark_test_curves::secp256k1::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField, Config, FrConfig};
use ark_test_curves::secp256k1::Fq;
pub struct Verifier {
    pub auth_session: AuthSession
}
pub fn init() -> Verifier {
            Verifier{
                auth_session: AuthSession{C: None, R: None, e: None, r:None, X:None}
            }
}
impl Verifier {
    pub fn gen_c(&mut self, random_value: Option<u64>){
        let mut random = 0;
        let mut small_rng = SmallRng::seed_from_u64(1);
        match random_value { 
            Some(rand_val) => {random = rand_val},
            None => {random  = small_rng.next_u64();}
        }
        
        let C = ScalarField::from(random);
        self.auth_session.C = Some(C);
    }
    pub fn serialize_c(&mut self) -> Option<String>{

        match self.auth_session.C {
            Some(C) => {
                //println!("Serializing c = {}",C.to_string());
                return Some(STANDARD.encode(C.to_string()))
            },
            None => {
                return None
            }
        }
        

    }

    pub fn consume_X(&mut self, X: String)
    {
        let decoded_X = STANDARD.decode(X);
        return match decoded_X {
            Err(e) => {
                panic!()
            }
            (X) => {
                let X_str = String::from_utf8(X.unwrap()).unwrap();
                
                let X_x_coord = Fq::from_str(&*X_str);

                let point = GAffine::get_point_from_x_unchecked(X_x_coord.unwrap(), false);
                //println!("Got point X {} from x coord {}", point.unwrap(), X_x_coord.unwrap());
                self.auth_session.X = point
            }
        }
    }
    pub fn consume_R(&mut self, R:String)
    {
        let decoded_R = STANDARD.decode(R);
        return match decoded_R {
            Err(e) => {
                panic!()
            }
            (R) => {
                let R_str = String::from_utf8(R.unwrap()).unwrap();


                let R_x_coord = Fq::from_str(&*R_str);

                let point = GAffine::get_point_from_x_unchecked(R_x_coord.unwrap(), true);
                //println!("Got point R {} from x coord {}", point.unwrap(), R_x_coord.unwrap());
                self.auth_session.R = point
            }
        }
    }
    
    pub fn verify_e(&mut self, e: String) -> bool {

        let g = G::generator();

        let decoded_e = STANDARD.decode(e);
        return match decoded_e {
            Err(e) => {
                false
            }
            (E) => {
                let e_str = String::from_utf8(E.unwrap()).unwrap();

                let coord = ScalarField::from_str(&*e_str);
                //println!("Multiplying point {} by scalar {}", g, coord.unwrap());
                let eG = g.mul(coord.unwrap());
                
                let cX = self.auth_session.X.unwrap().mul(self.auth_session.C.unwrap());
                //println!("{}{}", eG, self.auth_session.R.unwrap() + cX);
                eG == self.auth_session.R.unwrap() + cX
            }
        }
            
        

    }
}
