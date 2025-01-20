use std::error::Error;
use std::{thread, time};
use svcpasswdgen::cli_parser::get_config;
use svcpasswdgen::clipboard::{clear_clipboard, paste_to_clipboard};
use svcpasswdgen::password::{
    build_password, create_argon2_hash, create_argon2_salt, first120_from_full_sha512_hash,
    hash_cli_args,
};
use zeroize::Zeroize;

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = get_config()?;

    // build the hashes over the input values
    let mut password_hashes = hash_cli_args(&cli_args);

    // build sha512 sum over all parts
    let mut first120 = first120_from_full_sha512_hash(&password_hashes);
    password_hashes.zeroize();

    // build argon2id hash
    let salt = create_argon2_salt(&cli_args);
    let mut argon2_hash = create_argon2_hash(&first120, &salt);
    first120.zeroize();

    // build the final password
    let mut password = build_password(&cli_args, &argon2_hash);
    argon2_hash.zeroize();

    match cli_args.flags.paste_password_to_clipboard {
        false => {
            println!("{}", &password);
            password.zeroize();
        }
        true => {
            paste_to_clipboard(&password)?;
            println!(
                "Waiting {} seconds before clearing the clipboard.",
                cli_args.delay
            );
            thread::sleep(time::Duration::from_secs(cli_args.delay));
            clear_clipboard(&password)?;
            password.zeroize();
        }
    };

    Ok(())
}
