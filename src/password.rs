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
    // echo "hello, world"|openssl dgst -sha512                                                                                1 ↵
    // SHA2-512(stdin)= f65f341b35981fda842b09b2c8af9bcdb7602a4c2e6fa1f7d41f0974d3e3122f268fc79d5a4af66358f5133885cd1c165c916f80ab25e5d8d95db46f803c782c
    // # or
    // echo "hello, world"|sha512sum|cut -f 1 -d " "
    // f65f341b35981fda842b09b2c8af9bcdb7602a4c2e6fa1f7d41f0974d3e3122f268fc79d5a4af66358f5133885cd1c165c916f80ab25e5d8d95db46f803c782c
    let hex_result: String = get_sha512_digest("hello, world")
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    assert_eq!("f65f341b35981fda842b09b2c8af9bcdb7602a4c2e6fa1f7d41f0974d3e3122f268fc79d5a4af66358f5133885cd1c165c916f80ab25e5d8d95db46f803c782c", hex_result);
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
    let digest_result: String = digest::digest(&digest::SHA512, password_hashes)
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
