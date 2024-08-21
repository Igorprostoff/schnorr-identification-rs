use alloc::string::{String, ToString};
use core::ops::Mul;
use core::str::FromStr;
use crate::auth_session::AuthSession;
use base64::{ Engine, engine::general_purpose::STANDARD};
use ark_ec::{AffineRepr, PrimeGroup, CurveGroup, VariableBaseMSM};
use ark_test_curves::secp256k1::{G1Projective as G, Fr as ScalarField, Fq};
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

pub struct Prover {
    //curve: T,
    pub auth_session: AuthSession,
    private_key: String
}

pub fn init(password_hash: &str) -> Prover {

    let g = G::generator();

    let password_scalar = ScalarField::from_str(password_hash).unwrap();
    let p_multiplied = g * password_scalar;
    let p_multiplied_affine = p_multiplied.into_affine();

            Prover{
                //curve,
                private_key: password_hash.parse().unwrap(),
                auth_session: AuthSession{C: None, R: None, e: None, r:None, X: Some(p_multiplied_affine)}
            }


    
}
impl Prover {
    pub fn gen_R(&mut self, random_input: Option<u64>){
        let mut random: u64 = 0;
        let mut small_rng = SmallRng::seed_from_u64(1);
        match random_input {
            None => {random  = small_rng.next_u64();}
            Some(r) => {random = r}
        }
        
        let g = G::generator();
        let r_scalar = ScalarField::from(random);
        let r_multiplied = g.mul(r_scalar);
        let r_multiplied_affine = r_multiplied.into_affine();
        self.auth_session.R = Some(r_multiplied_affine);
        self.auth_session.r = Some(r_scalar);
    }
    
    pub fn serialize_R(&mut self) -> Option<String>{
        let mut R_x_coord: Fq;

        match self.auth_session.R {
            Some(R) => {
                
                //println!("Serialized R x coord of point {}", R);
                Some(STANDARD.encode(R.x().unwrap().to_string()))
            },
            None => {
                return None
            }
        }

    }

    pub fn serialize_X(&mut self) -> Option<String>{
        match self.auth_session.X {
            Some(X) => {
                //println!("Serializing X x coord {}", X);
                Some(STANDARD.encode(X.x().unwrap().to_string()))
            },
            None => {
                None
            }
        }
    }

    pub fn consume_c(&mut self, c: String) -> bool
    {
        let decoded_c = STANDARD.decode(c);
        return match decoded_c {
            Err(e) => {
                false
            }
            (R) => {

                let c_str = String::from_utf8(R.unwrap()).unwrap();
                //println!("Decoded c {}", c_str);
                let coord = ScalarField::from_str(&*c_str);


                self.auth_session.C = Some(coord.unwrap());
                true
            }
        }
        
    }


    /*fn consume_c_decoded(&mut self, c: Fp256<MontBackend<FrConfig, 4>>)
    {
        self.auth_session.C = Some(c);
    }*/
    
    pub fn gen_e(&mut self)
    {
        let private_key_scalar = ScalarField::from_str(&*self.private_key);
        //println!("Creating scalar from private key {} with result {}", self.private_key, private_key_scalar.unwrap());

        let e_scalar = self.auth_session.r.unwrap() + (self.auth_session.C.unwrap() * private_key_scalar.unwrap());

        self.auth_session.e = Some(e_scalar);
    }

    pub fn serialize_e(&mut self) -> Option<String>{
            match self.auth_session.e {
                Some(E) => {
                    //println!("Serializing E {} with result {}", E, E.to_string());
                    return Some(STANDARD.encode(E.to_string()))
                },
                None => {
                    return None
                }
            }
        }
}