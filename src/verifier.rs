use alloc::string::{String, ToString};
use core::ops::Mul;
use core::str::FromStr;
use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
use ark_ff::{Field, PrimeField};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use crate::auth_session::AuthSession;
use ark_test_curves::secp256k1::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField, Config, FrConfig, Fr};
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
    pub fn serialize_c(&mut self) -> Option<[u8;34]>{
        match self.auth_session.C {
            Some(C) => {
                //println!("Serializing c = {}",C.to_string());
                let mut result: [u8; 34] = [0; 34];
                result[0] = 2;
                result[1] = 32;
                for i in 0..4 {
                    let x = C.into_bigint().0[i].to_le_bytes();
                    for j in 0..8 {
                        result[2+i*8+j] = x[j];
                    }
                }
                return Some(result)
            },
            None => {
                return None
            }
        }
    }

    pub fn consume_X(&mut self, X: [u8;66])
    {
        if X[0] != 0x4 {
            return;
        }
        if X[1] != 0x40 {
            return;
        }
        let X_x_coord = Fq::from_random_bytes(&(X[2..34])).unwrap();
        //println!("Got x coord {:#}", X_x_coord);
        let X_y_coord = Fq::from_random_bytes(&(X[34..66])).unwrap();
        //println!("Got y coord {:#}", X_y_coord);
        let mut point_greatest = GAffine::get_point_from_x_unchecked(X_x_coord, true);
        let mut point_least = GAffine::get_point_from_x_unchecked(X_x_coord, false);
        
        match point_greatest { 
            Some(point) => {
                if point.y().unwrap() == X_y_coord {
                    self.auth_session.X = Some(point);
                }
                
            }
            None => {/*println!("Unable to compute point_greatest")*/}
        }
        
        match point_least { 
            Some(point) => {
                if point.y().unwrap() == X_y_coord {
                    self.auth_session.X = Some(point);
                }
            }
            None => {/*println!("Unable to compute point_least")*/}
        }

    }
    pub fn consume_R(&mut self, R:[u8;66]) -> bool
    {
        if R[0] != 0x4 {
            return false;
        }
        if R[1] != 0x40 {
            return false;
        }
        let R_x_coord = Fq::from_random_bytes(&(R[2..34])).unwrap();
        //println!("Got x coord {:#}", R_x_coord);
        let R_y_coord = Fq::from_random_bytes(&(R[34..66])).unwrap();
        //println!("Got y coord {:#}", R_y_coord);
        let mut point_greatest = GAffine::get_point_from_x_unchecked(R_x_coord, true);
        let mut point_least = GAffine::get_point_from_x_unchecked(R_x_coord, false);

        match point_greatest {
            Some(point) => {
                if point.y().unwrap() == R_y_coord {
                    self.auth_session.R = Some(point);
                    return true;
                }

            }
            None => {/*println!("Unable to compute point_greatest")*/}
        }

        match point_least {
            Some(point) => {
                if point.y().unwrap() == R_y_coord {
                    self.auth_session.R = Some(point);
                    return true;
                }
            }
            None => {/*println!("Unable to compute point_least");*/}
        }
        return false
    }
    
    pub fn verify_e(&mut self, e: [u8;34]) -> bool {

        let g = G::generator();
        if e[0] != 0x2 {
            false;
        }
        if e[1] != 0x20 {
            false;
        }
        let e = Fr::from_random_bytes(&(e[2..34])).unwrap();
        //println!("Decoded c {}", c_str);
        //println!("Multiplying point {} by scalar {}", g, coord.unwrap());
        let eG = g.mul(e);
        let cX = self.auth_session.X.unwrap().mul(self.auth_session.C.unwrap());
        //println!("{}{}", eG, self.auth_session.R.unwrap() + cX);
        eG == self.auth_session.R.unwrap() + cX
    }
}
