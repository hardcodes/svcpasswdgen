use crate::{base64_trait::Base64StringConversions, cli_parser::CliArguments};
use crate::{
    ARGON2_MEMORY_COST, ARGON2_NUM_ITERATIONS, ARGON2_NUM_PARALLEL_THREADS, ARGON2_OUTPUT_LENGTH,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Algorithm, Argon2, Params, Version,
};
use base64::{engine::general_purpose, Engine as _};
use ring::digest;

/// build sha512 hash from the given value.
fn get_sha512_digest(input: &str) -> Vec<u8> {
    let digest = digest::digest(&digest::SHA512, input.as_bytes());
    digest.as_ref().to_vec()
}

#[test]
fn get_sha512_digest_hello_world() {
    // what we expect:
    //
    // ```bash
    // echo -n "hello, world"|openssl dgst -sha512                                                                                1 ↵
    // SHA2-512(stdin)= 8710339dcb6814d0d9d2290ef422285c9322b7163951f9a0ca8f883d3305286f44139aa374848e4174f5aada663027e4548637b6d19894aec4fb6c46a139fbf9
    // # or
    // echo -n "hello, world"|sha512sum|cut -f 1 -d " "
    // 8710339dcb6814d0d9d2290ef422285c9322b7163951f9a0ca8f883d3305286f44139aa374848e4174f5aada663027e4548637b6d19894aec4fb6c46a139fbf9
    let hex_result: String = get_sha512_digest("hello, world")
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    assert_eq!("8710339dcb6814d0d9d2290ef422285c9322b7163951f9a0ca8f883d3305286f44139aa374848e4174f5aada663027e4548637b6d19894aec4fb6c46a139fbf9", hex_result);
}

#[test]
fn get_sha512_digest_server001() {
    let hex_result: String = get_sha512_digest("server001")
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    assert_eq!("160cd0d973e5b5ce5c245b9338e1ee0016c7b4d5f162a2596f5ed2d04bcee21095921b1f22936b3c700c4ed092001f74a54d9ce8ae83c50dbb5ea2e91562e005", hex_result);
}

#[test]
fn get_sha512_digest_superuser() {
    let hex_result: String = get_sha512_digest("superuser")
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    assert_eq!("2cff38a527697f0c8df41a644671718d7d139c9b6d836e126b62677d8b57b1598874b6b0595c10358f59ca4e943d8fd2aa57327db011a421a80ec65945ea210b", hex_result);
}

#[test]
fn get_sha512_digest_passw0rd() {
    let hex_result: String = get_sha512_digest("passw0rd")
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    assert_eq!("e0469addd8d57a3623494096dabc19bebca1a038c9da696940b3f853d106a6ecfa5bd60ce8e72884efa3bd92b930da178fd616f40facad654212d7c2f8817dd4", hex_result);
}

#[test]
fn get_sha512_digest_server001superuser() {
    let hex_result: String = get_sha512_digest("server001superuser")
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    assert_eq!("cddab6bb2eaeaef88d39998fbd3aba5a88d7c378c300cdbcc162c5dedc3472de33873987731e95c7c0f0642a9ffefe7140ca255e797875015abb74a28f746125", hex_result);
}

/// build sha512 hashes over all input values
pub fn hash_cli_args(cli_args: &CliArguments) -> Vec<u8> {
    // build the hashes over the input values
    let mut passwd_hashes: Vec<u8> = Vec::new();
    passwd_hashes.append(&mut get_sha512_digest(&cli_args.machine));
    passwd_hashes.append(&mut get_sha512_digest(&cli_args.account));
    passwd_hashes.append(&mut get_sha512_digest(&cli_args.seed));
    if cli_args.extra.is_some() {
        for extra in cli_args.extra.as_ref().unwrap() {
            passwd_hashes.append(&mut get_sha512_digest(extra));
        }
    }
    passwd_hashes
}

/// build a sha512 hash over all password hashes
/// and return the first 120 characters as `String`.
pub fn first120_from_full_sha512_hash(password_hashes: &[u8]) -> String {
    //convert binary hashes to string representation for same result as in cli
    let hash: String = password_hashes
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    let digest_result: String = digest::digest(&digest::SHA512, hash.as_bytes())
        .as_ref()
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    let first120 = &digest_result[..120];
    first120.to_string()
}

/// Create a salt for the argon2 hash.
/// Use input values `--machine` and `--account`.
pub fn create_argon2_salt(cli_args: &CliArguments) -> String {
    let mut salt = cli_args.machine.to_owned();
    salt.push_str(&cli_args.account);
    let mut passwd_hash: Vec<u8> = Vec::new();
    passwd_hash.append(&mut get_sha512_digest(&salt));
    let hash: String = passwd_hash.iter().map(|x| format!("{:02x}", &x)).collect();
    let first32 = &hash[..32];
    // mimic the argon2 command and encode the password before using it.
    first32.to_base64_encoded_no_padding()
}

/// build an argon2 hash
pub fn create_argon2_hash(input: &str, salt: &str) -> String {
    let params = Params::new(
        ARGON2_MEMORY_COST,
        ARGON2_NUM_ITERATIONS,
        ARGON2_NUM_PARALLEL_THREADS,
        Some(ARGON2_OUTPUT_LENGTH),
    )
    .unwrap();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let salt_from_b64 = SaltString::from_b64(salt).unwrap();
    let argon2_password_hash = argon2
        .hash_password(input.as_bytes(), salt_from_b64.as_salt())
        .unwrap();
    general_purpose::STANDARD_NO_PAD.encode(argon2_password_hash.hash.unwrap().as_bytes())
}

/// build password, of prefix + base64 encoded argon2 hash + suffix
pub fn build_password(cli_args: &CliArguments, argon2_hash: &str) -> String {
    let argon2_hash_sha512 = get_sha512_digest(argon2_hash);
    let agron2_sha512_hash: String = argon2_hash_sha512
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    let mut password = cli_args.prefix.clone();
    let base64_digest_result = agron2_sha512_hash.to_base64_urlsafe_encoded();
    let used_digest_part = &base64_digest_result[..cli_args.length as usize];
    password.push_str(used_digest_part);
    password.push_str(&cli_args.suffix);
    password
}
