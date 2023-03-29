use crate::cli_parser::CliArguments;
use sha2::{Digest, Sha512};

/// build sha512 hash from the given value.
fn get_hashed(value: &str) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(value.as_bytes());
    let hash_result = hasher.finalize();
    hash_result.to_vec()
}

// build sha512 hashes over all input values
pub fn hash_cli_args(cli_args: &CliArguments) -> Vec<u8> {
    // build the hashes over the input values
    let mut passwd_hashes: Vec<u8> = Vec::new();
    passwd_hashes.append(&mut get_hashed(&cli_args.machine));
    passwd_hashes.append(&mut get_hashed(&cli_args.account));
    passwd_hashes.append(&mut get_hashed(&cli_args.seed));
    if cli_args.extra.is_some() {
        for extra in cli_args.extra.as_ref().unwrap() {
            passwd_hashes.append(&mut get_hashed(extra));
        }
    }
    passwd_hashes
}

/// build password, of prefix + result hash + suffix
pub fn build_password(cli_args: &CliArguments, password_hashes: &Vec<u8>) -> String {
    let mut passwd_hasher = Sha512::new();
    passwd_hasher.update(password_hashes);
    let result = passwd_hasher.finalize();
    let result = format!("{:0x}", &result);
    let mut password = cli_args.prefix.clone();
    let used_hash = &result[..cli_args.length as usize];
    password.push_str(used_hash);
    password.push_str(&cli_args.suffix);
    password
}
