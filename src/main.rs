use std::error::Error;
use svcpasswdgen::cli_parser::get_config;
use sha2::{Sha512,Digest};

/// build sha521 hash from the given value.
pub fn get_hashed(value: &str) -> Vec<u8>{
    let mut hasher = Sha512::new();
    hasher.update(value.as_bytes());
    let hash_result = hasher.finalize();
    let mut v = Vec::with_capacity(hash_result.len());
    for index in 0..hash_result.len(){
        v.push(hash_result[index]);
    }
    v
}
fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = get_config()?;

    // build the hashes over the input values
    let mut passwd_hashes: Vec<u8> = Vec::new();
    passwd_hashes.append(&mut get_hashed(&cli_args.machine));
    passwd_hashes.append(&mut get_hashed(&cli_args.account));
    passwd_hashes.append(&mut get_hashed(&cli_args.seed));
    if cli_args.extra.is_some(){
        for extra in cli_args.extra.unwrap(){
            passwd_hashes.append(&mut get_hashed(&extra));
        }
    }
    // build result hash
    let mut passwd_hasher = Sha512::new();
    passwd_hasher.update(passwd_hashes);
    let result = passwd_hasher.finalize();
    let result = format!("{:0x}", &result);
    // build password, of prefix + result hash + suffix
    let mut password = cli_args.prefix;
    let used_hash = &result[..cli_args.length as usize];
    password.push_str(used_hash);
    password.push_str(&cli_args.suffix);
    println!("{}", &password);

    Ok(())
}

