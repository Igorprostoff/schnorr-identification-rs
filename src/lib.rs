#![no_std]

extern crate alloc;

pub mod prover;
pub mod config;
pub mod verifier;
pub mod auth_session;

#[cfg(test)]
mod tests {
    use crate::{prover, verifier};
    use ark_test_curves::secp256k1::Config;
    #[test]
    fn init_prover() {
        
        prover::init( "1234");
    }

    #[test]
    fn gen_r() {
        
        let mut p = prover::init( "1234");
        p.gen_R(None);
    }

    #[test]
    fn serialize_r() {
        let mut p = prover::init("1234");
        p.gen_R(Some(12792143));
        assert_eq!("MTkzNjU4ODIxMDAwODAzOTgyNDgyNjUyNjU3NTMyMTE1MTAzNTcyNDU4NDk5Nzc4NzQ3MjM2MDYyNzUzNjQ1MTIxMjM1NDQ1MzM3MDg=", p.serialize_R().unwrap())
    }

    #[test]
    fn serialize_X() {
        let mut p = prover::init( "1234");
        assert_eq!("MTAyODg0MDAzMzIzODI3MjkyOTE1NjY4MjM5NzU5OTQwMDUzMTA1OTkyMDA4MDg3NTIwMjA3MTUwNDc0ODk2MDU0MTg1MTgwNDIwMzM4", p.serialize_X().unwrap())
    }

    #[test]
    fn init_verifier() {
        
        let v = verifier::init();
        
    }

    #[test]
    fn consume_X() {

        let mut p = prover::init( "1234");
        p.gen_R(None);
        let serialized_X = p.serialize_X();

        let mut v = verifier::init();
        
        v.consume_X(serialized_X.unwrap());

    }

    #[test]
    fn consume_R() {

        let mut p = prover::init( "1234");
        p.gen_R(None);
        let serialized_R = p.serialize_R();


        let mut v = verifier::init();
        v.consume_R(serialized_R.unwrap())
    }

    #[test]
    fn gen_c() {

        let mut p = prover::init( "1234");
        p.gen_R(None);
        let serialized_R = p.serialize_R();
        let serialized_X = p.serialize_X();


        let mut v = verifier::init();
        v.consume_R(serialized_R.unwrap());
        v.consume_X(serialized_X.unwrap());
        v.gen_c(None);
        
    }

    #[test]
    fn serialize_c() {

        let mut p = prover::init( "1234");
        p.gen_R(None);
        let serialized_R = p.serialize_R();
        let serialized_X = p.serialize_X();

        let mut v = verifier::init();
        v.consume_R(serialized_R.unwrap());
        v.consume_X(serialized_X.unwrap());
        v.gen_c(Some(5039));
        assert_eq!("NTAzOQ==", v.serialize_c().unwrap())

    }
    #[test]
    fn consume_c() {

        let mut p = prover::init( "1234");
        p.gen_R(None);
        let serialized_R = p.serialize_R();
        let serialized_X = p.serialize_X();

        let mut v = verifier::init();
        v.consume_R(serialized_R.unwrap());
        v.consume_X(serialized_X.unwrap());
        v.gen_c(None);
        let c_serialized = v.serialize_c();
        
        assert!(p.consume_c(c_serialized.unwrap()))
    }
    

    #[test]
    fn gen_e() {

        let mut p = prover::init( "1234");
        p.gen_R(None);
        let serialized_R = p.serialize_R();
        let serialized_X = p.serialize_X();

        let mut v = verifier::init();
        v.consume_R(serialized_R.unwrap());
        v.consume_X(serialized_X.unwrap());
        v.gen_c(None);
        let c_serialized = v.serialize_c();

        p.consume_c(c_serialized.unwrap());
        p.gen_e();
    }

    #[test]
    fn serialize_e() {

        let mut p = prover::init( "1234");
        p.gen_R(Some(12792143));
        let serialized_R = p.serialize_R();
        let serialized_X = p.serialize_X();

        let mut v = verifier::init();
        v.consume_R(serialized_R.unwrap());
        v.consume_X(serialized_X.unwrap());
        v.gen_c(Some(5039));
        let c_serialized = v.serialize_c();

        p.consume_c(c_serialized.unwrap());
        p.gen_e();
        assert_eq!("MTkwMTAyNjk=", p.serialize_e().unwrap())
    }
    #[test]
    fn verify_proof() {

        let mut p = prover::init( "1234");
        p.gen_R(None);
        let r_serialized = p.serialize_R();
        let x_serialized = p.serialize_X();
        let mut v = verifier::init();
        v.consume_R(r_serialized.unwrap());
        v.consume_X(x_serialized.unwrap());
        v.gen_c(None);
        let c_serialized = v.serialize_c();
        p.consume_c(c_serialized.unwrap());
        p.gen_e();
        let e_serialized = p.serialize_e();
        
        
        assert!(v.verify_e(e_serialized.unwrap()));
        
    }
    
}
