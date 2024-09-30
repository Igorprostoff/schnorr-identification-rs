#![no_std]
#[macro_use]
extern crate alloc;

pub mod prover;
pub mod config;
pub mod verifier;
pub mod auth_session;

#[cfg(test)]
mod tests {
    use ark_ec::AffineRepr;
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
         assert_eq!([4, 64, 204, 250, 93, 42, 64, 12, 38, 174, 133, 28, 15, 18, 137, 235, 116, 66, 9, 38, 213, 96, 165, 30, 215, 250, 255, 224, 181, 253, 105, 179, 208, 42, 94, 21, 152, 39, 168, 146, 193, 181, 218, 154, 174, 207, 2, 128, 248, 58, 110, 6, 33, 123, 2, 97, 150, 165, 129, 99, 38, 172, 236, 0, 11, 195], p.serialize_R().unwrap())
     }
 
     #[test]
     fn serialize_X() {
         let mut p = prover::init( "1234");
         assert_eq!([4, 64, 242, 88, 22, 63, 101, 246, 88, 101, 167, 154, 66, 121, 226, 235, 171, 181, 165, 123, 133, 80, 29, 212, 179, 129, 209, 220, 96, 92, 67, 72, 118, 227, 76, 48, 139, 211, 241, 143, 6, 45, 92, 192, 127, 52, 148, 140, 237, 130, 249, 167, 111, 156, 62, 101, 174, 100, 241, 88, 65, 45, 168, 233, 46, 109],
                    p.serialize_X().unwrap())
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
         assert_eq!(v.auth_session.X.unwrap().x().unwrap(), p.auth_session.X.unwrap().x().unwrap());
         assert_eq!(v.auth_session.X.unwrap().y().unwrap(), p.auth_session.X.unwrap().y().unwrap());
 
     }
     

    #[test]
    fn consume_R() {

        let mut p = prover::init( "1234");
        p.gen_R(None);
        let serialized_R = p.serialize_R();


        let mut v = verifier::init();
        assert!(v.consume_R(serialized_R.unwrap()));
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
        assert_eq!([2, 32, 175, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], v.serialize_c().unwrap())

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
        assert_eq!([2, 32, 221, 18, 34, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], p.serialize_e().unwrap())
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
