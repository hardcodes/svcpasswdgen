use crate::cli_parser::CliArguments;
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

// build sha512 hashes over all input values
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

/// build password, of prefix + result hash + suffix
pub fn build_password(cli_args: &CliArguments, password_hashes: &Vec<u8>) -> String {
    //convert backup to string for same result as in cli
    let hash: String = password_hashes
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    let digest_result: String = digest::digest(&digest::SHA512, hash.as_bytes())
        .as_ref()
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    let mut password = cli_args.prefix.clone();
    let used_hash = &digest_result[..cli_args.length as usize];
    password.push_str(used_hash);
    password.push_str(&cli_args.suffix);
    password
}
