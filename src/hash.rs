use rand::Rng;

use hex;
use sha2::{Sha256, Digest};
use argon2::{self, Config};
use uuid::Uuid;

use crate::settings;

pub fn get_sha256<T: std::convert::AsRef<[u8]> + ?Sized>(to_hash: &T) -> Vec<u8> {

	let mut hasher = Sha256::new();
	hasher.update(to_hash);
	hasher.finalize().to_vec()

}

pub fn get_sha256_hex<T: std::convert::AsRef<[u8]> + ?Sized>(to_hash: &T) -> String {

	hex::encode(get_sha256(to_hash))

}

pub fn get_argon2(to_hash: &str, salt: &str) -> Vec<u8> {

	let config = Config::default();
	argon2::hash_raw(to_hash.as_bytes(), salt.as_bytes(), &config).unwrap()

}

pub fn get_peppered_argon2(to_hash: &str, salt: &str) -> Vec<u8> {

	let config = Config::default();
	let mut peppered_with_salt = String::from(salt);
	peppered_with_salt.push_str(&settings::get_setting("argon2_pepper"));
	argon2::hash_raw(to_hash.as_bytes(), peppered_with_salt.as_bytes(), &config).unwrap()

}

pub fn get_argon2_hex(to_hash: &str, salt: &str) -> String {

	return hex::encode(get_argon2(to_hash, salt));

}

pub fn get_random_string32() -> String {

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const RAND_STRING_LEN: usize = 32;
    let mut rng = rand::thread_rng();

    let rand_string: String = (0..RAND_STRING_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    rand_string
}

pub fn get_random_string(length: i32) -> String {

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let rand_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    rand_string
}

pub fn get_uuid() -> String {

	Uuid::new_v4().to_simple().to_string()

}