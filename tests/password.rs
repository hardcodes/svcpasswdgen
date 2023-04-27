use ring::digest;
use svcpasswdgen::cli_parser::{CliArguments, DEFAULT_PREFIX, DEFAULT_SUFFIX};
use svcpasswdgen::password::{
    build_password, create_argon2_hash, create_argon2_salt, first120_from_full_sha512_hash,
    hash_cli_args,
};

const SERVER001: &str = "server001";
const SUPERUSER: &str = "superuser";
const PASSW0RD: &str = "passw0rd";

#[test]
fn hash_cli_args_server001superuserpassw0rd() {
    let cli_args = CliArguments {
        prefix: DEFAULT_PREFIX.to_string(),
        suffix: DEFAULT_SUFFIX.to_string(),
        machine: SERVER001.to_string(),
        account: SUPERUSER.to_string(),
        seed: PASSW0RD.to_string(),
        length: 20,
        extra: None,
    };
    let hash: String = hash_cli_args(&cli_args)
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    assert_eq!("160cd0d973e5b5ce5c245b9338e1ee0016c7b4d5f162a2596f5ed2d04bcee21095921b1f22936b3c700c4ed092001f74a54d9ce8ae83c50dbb5ea2e91562e0052cff38a527697f0c8df41a644671718d7d139c9b6d836e126b62677d8b57b1598874b6b0595c10358f59ca4e943d8fd2aa57327db011a421a80ec65945ea210be0469addd8d57a3623494096dabc19bebca1a038c9da696940b3f853d106a6ecfa5bd60ce8e72884efa3bd92b930da178fd616f40facad654212d7c2f8817dd4", hash);
}

#[test]
fn build_password_server001superuserpassw0rd() {
    let cli_args = CliArguments {
        prefix: DEFAULT_PREFIX.to_string(),
        suffix: DEFAULT_SUFFIX.to_string(),
        machine: SERVER001.to_string(),
        account: SUPERUSER.to_string(),
        seed: PASSW0RD.to_string(),
        length: 20,
        extra: None,
    };
    let hash: String = hash_cli_args(&cli_args)
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();

    let digest_result: String = digest::digest(&digest::SHA512, hash.as_bytes())
        .as_ref()
        .iter()
        .map(|x| format!("{:02x}", &x))
        .collect();
    assert_eq!("bc362aa50b489f0f4fc6594aca3a6b24093fb507d7813e15493ca791a2fe2e12fcefd91fa15a5149884d30e3b0a6aebd734d55a7a12559b66aa93f3a675fa71d", digest_result);

    let password_hashes = hash_cli_args(&cli_args);
    let first120 = first120_from_full_sha512_hash(&password_hashes);
    assert_eq!("bc362aa50b489f0f4fc6594aca3a6b24093fb507d7813e15493ca791a2fe2e12fcefd91fa15a5149884d30e3b0a6aebd734d55a7a12559b66aa93f3a", first120);

    let salt = create_argon2_salt(&cli_args);
    assert_eq!("Y2RkYWI2YmIyZWFlYWVmODhkMzk5OThmYmQzYWJhNWE", salt);

    let argon2_hash = create_argon2_hash(&first120, &salt);
    assert_eq!("hlfAJAVBleLuYqnXJmO/3adUAYGHvMtghtYYlBb0c7I", argon2_hash);

    let password = build_password(&cli_args, &argon2_hash);
    assert_eq!("Pr3YWQ0ZjE2ZDZlOWYxMjkw$1X", password);
}
