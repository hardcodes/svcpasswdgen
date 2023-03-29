use std::error::Error;
use svcpasswdgen::cli_parser::get_config;
use sha2::{Sha512,Digest};

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = get_config()?;

    let mut hasher = Sha512::new();
    let mut passwd_source: String = cli_args.machine;
    passwd_source.push_str(&cli_args.account);
    passwd_source.push_str(&cli_args.seed);
    if cli_args.extra.is_some(){
        for extra in cli_args.extra.unwrap(){
            passwd_source.push_str(&extra);
        }
    }
    hasher.update(passwd_source.as_bytes());
    let result = hasher.finalize();
    let result = format!("{:0x}", &result);
    let mut password = cli_args.prefix;
    let used_hash = &result[..cli_args.length as usize];
    password.push_str(used_hash);
    password.push_str(&cli_args.suffix);
    println!("{}", &password);

    Ok(())
}

