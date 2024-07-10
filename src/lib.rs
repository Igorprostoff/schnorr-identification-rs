mod prover;
mod config;
mod verifier;
mod auth_session;

#[cfg(test)]
mod tests {
    use p256::NistP256;
    use crate::{prover, verifier};
    use elliptic_curve::rand_core;
    use rand_core::CryptoRngCore;

    #[test]
    fn init_prover() {
        prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
    }

    #[test]
    fn gen_r() {
        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(None);
    }

    #[test]
    fn serialize_r() {
        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(Some(12792143));
        assert_eq!("UQ8b/ije/+57aFWy0tWqAuhBxLylCMQJKcY4+ZFrATw=", p.serialize_R().unwrap())
    }

    #[test]
    fn serialize_X() {
        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        assert_eq!("9y9f39e3yfLMdC2Z0JIk6xrTcDISMt841DSNpoGjjuc=", p.serialize_X().unwrap())
    }

    #[test]
    fn init_verifier() {
        
        let mut v = verifier::init(NistP256);
        
        
    }

    #[test]
    fn consume_X() {

        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(None);


        let mut v = verifier::init(NistP256);
        v.consume_X(p.auth_session.X.unwrap());

    }

    #[test]
    fn consume_R() {

        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(None);


        let mut v = verifier::init(NistP256);
        v.consume_R(p.auth_session.R.unwrap())
    }

    #[test]
    fn gen_c() {

        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(None);


        let mut v = verifier::init(NistP256);
        v.consume_R(p.auth_session.R.unwrap());
        v.consume_X(p.auth_session.X.unwrap());
        v.gen_c(None);
        
    }

    #[test]
    fn serialize_c() {

        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(None);


        let mut v = verifier::init(NistP256);
        v.consume_R(p.auth_session.R.unwrap());
        v.consume_X(p.auth_session.X.unwrap());
        v.gen_c(Some(5039));
        assert_eq!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAE68=", v.serialize_c().unwrap())

    }
    #[test]
    fn consume_c_decoded() {

        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(None);


        let mut v = verifier::init(NistP256);
        v.consume_R(p.auth_session.R.unwrap());
        v.consume_X(p.auth_session.X.unwrap());
        v.gen_c(None);
        
        p.consume_c_decoded(v.auth_session.C.unwrap())
    }

    #[test]
    fn consume_c() {

        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(None);


        let mut v = verifier::init(NistP256);
        v.consume_R(p.auth_session.R.unwrap());
        v.consume_X(p.auth_session.X.unwrap());
        v.gen_c(None);
        
        assert_eq!(true, p.consume_c(v.serialize_c().unwrap()))
    }

    #[test]
    fn gen_e() {

        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(None);


        let mut v = verifier::init(NistP256);
        v.consume_R(p.auth_session.R.unwrap());
        v.consume_X(p.auth_session.X.unwrap());
        v.gen_c(None);

        p.consume_c_decoded(v.auth_session.C.unwrap());
        p.gen_e();
    }

    #[test]
    fn serialize_e() {

        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(Some(12792143));


        let mut v = verifier::init(NistP256);
        v.consume_R(p.auth_session.R.unwrap());
        v.consume_X(p.auth_session.X.unwrap());
        v.gen_c(Some(5039));

        p.consume_c_decoded(v.auth_session.C.unwrap());
        p.gen_e();
        assert_eq!("c651f5aDOR768ZdgldfNnKfKSa0Ddm5sDOzElY2FOF8=", p.serialize_e().unwrap())
    }
    #[test]
    fn verify_proof() {

        let mut p = prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
        p.gen_R(None);


        let mut v = verifier::init(NistP256);
        v.consume_R(p.auth_session.R.unwrap());
        v.consume_X(p.auth_session.X.unwrap());
        v.gen_c(None);

        p.consume_c_decoded(v.auth_session.C.unwrap());
        p.gen_e();
        
        assert!(v.verify_e(p.auth_session.e.unwrap()));
        
    }
    
}
