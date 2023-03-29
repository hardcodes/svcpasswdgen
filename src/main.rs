use std::error::Error;
use svcpasswdgen::cli_parser::get_config;
use svcpasswdgen::password::{hash_cli_args, build_password};

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = get_config()?;

    // build the hashes over the input values
    let password_hashes = hash_cli_args(&cli_args);
    
    // build final password
    let password = build_password(&cli_args, &password_hashes);
    println!("{}", &password);

    Ok(())
}
