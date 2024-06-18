mod prover;
mod config;
mod verifier;
mod auth_session;

#[cfg(test)]
mod tests {
    use p256::NistP256;
    use crate::prover;

    #[test]
    fn init_prover() {
        prover::init(NistP256, "8e38fc4ffe677662dde8e1a63fbcd45959d2a4c3004d27e98c4fedf2d0c14c01");
    }
    
}
