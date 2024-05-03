use pretty_assertions::{assert_eq, assert_ne};
use rand::distributions::{Alphanumeric, DistString};

use crate::services::totp_service::make_time_otp;

#[test]
pub fn test_create_time_otp() {
    let mut rng = rand::thread_rng();
    let secret = Alphanumeric.sample_string(&mut rng, 16);
    let build = make_time_otp(secret.as_str(), 60, 0).unwrap();
    let build_again = make_time_otp(secret.as_str(), 60, 0).unwrap();

    assert_eq!(build, build_again);
}

#[test]
pub fn test_check_time_otp() {
    let mut rng = rand::thread_rng();
    let secret = Alphanumeric.sample_string(&mut rng, 16);
    let build = make_time_otp(secret.as_str(), 3600, 0).unwrap();
    let build_again = make_time_otp(secret.as_str(), 90, 0).unwrap();

    assert_ne!(build, build_again);
}
