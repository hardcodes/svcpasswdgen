use std::error::Error;
use svcpasswdgen::cli_parser::get_config;
use svcpasswdgen::clipboard::paste_to_clipboard;
use svcpasswdgen::password::{
    build_password, create_argon2_hash, create_argon2_salt, first120_from_full_sha512_hash,
    hash_cli_args,
};

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = get_config()?;

    // build the hashes over the input values
    let password_hashes = hash_cli_args(&cli_args);

    // build sha512 sum over all parts
    let first120 = first120_from_full_sha512_hash(&password_hashes);

    // build argon2id hash
    let salt = create_argon2_salt(&cli_args);
    let argon2_hash = create_argon2_hash(&first120, &salt);

    // build the final password
    let password = build_password(&cli_args, &argon2_hash);
    match cli_args.flags.paste_password_to_clipboard {
        false => {
            println!("{}", &password);
        }
        true => {
            paste_to_clipboard(&password)?;
        }
    };

    Ok(())
}
