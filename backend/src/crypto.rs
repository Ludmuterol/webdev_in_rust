use std::num::NonZeroU32;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};

const PEPPER_FILE: &[u8] = include_str!("../pepper.txt").as_bytes();

pub fn encrypt(secret: String) -> (String, String) {
    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rand::SystemRandom::new().fill(&mut salt).unwrap();
    let mut salt_pepper: Vec<u8> = salt.to_vec();
    salt_pepper.extend(PEPPER_FILE);
    let mut pbkdf2_hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA512, NonZeroU32::new(210_000).unwrap(), &salt_pepper, secret.as_bytes(), &mut pbkdf2_hash);
    (HEXUPPER.encode(&salt), HEXUPPER.encode(&pbkdf2_hash))
}

pub fn verify(input: String, salt_encoded: &String, actual_hash: &String) -> Result<(), ()> {
    let salt = HEXUPPER.decode(salt_encoded.as_bytes()).unwrap();
    let mut salt_pepper: Vec<u8> = salt.to_vec();
    salt_pepper.extend(PEPPER_FILE);
    let previously_derived = HEXUPPER.decode(actual_hash.as_bytes()).unwrap();
    let res = pbkdf2::verify(pbkdf2::PBKDF2_HMAC_SHA512, NonZeroU32::new(210_000).unwrap(), &salt_pepper, input.as_bytes(), &previously_derived);
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
