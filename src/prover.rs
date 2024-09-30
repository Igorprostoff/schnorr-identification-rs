use alloc::string::{String, ToString};
use core::ops::Mul;
use core::str::{FromStr};
use bytes::Bytes;

use crate::auth_session::AuthSession;
use base64::{ Engine, engine::general_purpose::STANDARD};
use ark_ec::{AffineRepr, PrimeGroup, CurveGroup, VariableBaseMSM};
use ark_ff::{Field, PrimeField};
use ark_test_curves::secp256k1::{G1Projective as G, Fr as ScalarField, Fq, Fr};
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
    
    pub fn serialize_R(&mut self) -> Option<[u8;66]>{
        match self.auth_session.R {
            Some(R) => {

                //println!("Serializing X x coord {}", R);
                let mut result: [u8; 66] = [0; 66];
                result[0] = 4;
                result[1] = 64;
                for i in 0..4 {
                    let x = R.xy().unwrap().0.into_bigint().0[i].to_le_bytes();
                    for j in 0..8 {
                        result[2+i*8+j] = x[j];
                    }
                }

                for i in 0..4 {
                    let x = R.xy().unwrap().1.into_bigint().0[i].to_le_bytes();
                    for j in 0..8 {
                        result[2+32+i*8+j] = x[j];
                    }
                }
                return Some(result);
            },
            None => {
                None
            }
        }

    }

    pub fn serialize_X(&mut self) -> Option<[u8;66]>{
        match self.auth_session.X {
            Some(X) => {
                
                //println!("Serializing X x coord {}", X);
                let mut result: [u8; 66] = [0; 66];
                result[0] = 4;
                result[1] = 64;
                for i in 0..4 {
                    let x = X.xy().unwrap().0.into_bigint().0[i].to_le_bytes();
                    for j in 0..8 {
                        result[2+i*8+j] = x[j];
                    }
                }

                for i in 0..4 {
                    let x = X.xy().unwrap().1.into_bigint().0[i].to_le_bytes();
                    for j in 0..8 {
                        result[2+32+i*8+j] = x[j];
                    }
                }
                return Some(result);
            },
            None => {
                None
            }
        }
    }

    pub fn consume_c(&mut self, c: [u8;34]) -> bool
    {
        if c[0] != 0x2 {
            false;
        }
        if c[1] != 0x20 {
            false;
        }
        let c = Fr::from_random_bytes(&(c[2..34])).unwrap();
        //println!("Decoded c {}", c_str);
       
        self.auth_session.C = Some(c);
        true
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

    pub fn serialize_e(&mut self) -> Option<[u8;34]> {
        match self.auth_session.e {
            Some(e) => {
                //println!("Serializing c = {}",C.to_string());
                let mut result: [u8; 34] = [0; 34];
                result[0] = 2;
                result[1] = 32;
                for i in 0..4 {
                    let x = e.into_bigint().0[i].to_le_bytes();
                    for j in 0..8 {
                        result[2 + i * 8 + j] = x[j];
                    }
                }
                return Some(result)
            },
            None => {
                return None
            }
        }
    }
}