use pretty_assertions::{assert_eq, assert_ne};
use rand::distributions::{Alphanumeric, DistString};

use crate::services::hotp_service::make_hmac_otp;

#[test]
pub fn test_create_hmac_otp() {
    let mut rng = rand::thread_rng();
    let secret = Alphanumeric.sample_string(&mut rng, 16);
    let build = make_hmac_otp(secret.as_str(), 0).unwrap();
    let build_again = make_hmac_otp(secret.as_str(), 0).unwrap();

    assert_eq!(build, build_again);
}

#[test]
pub fn test_check_hmac_otp() {
    let mut rng = rand::thread_rng();
    let secret = Alphanumeric.sample_string(&mut rng, 16);
    let build = make_hmac_otp(secret.as_str(), 0).unwrap();
    let build_again = make_hmac_otp(secret.as_str(), 5).unwrap();

    assert_ne!(build, build_again);
}
