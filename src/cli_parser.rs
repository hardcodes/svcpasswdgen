use crate::{PROGRAM_AUTHORS, PROGRAM_DESCRIPTION, PROGRAM_NAME, PROGRAM_VERSION};
use clap::{value_parser, Arg};
use std::error::Error;
use std::fmt::Debug;
use std::str;

const DEFAULT_PREFIX: &str = "Pz";
const DEFAULT_SUFFIX: &str = "$";
const ARG_MACHINE_NAME: &str = "arg-machine-name";
const ARG_ACCOUNT_NAME: &str = "arg-account-name";
const ARG_SEED_PASSWORD: &str = "arg-seed-password";
const ARG_EXTRA_VALUE: &str = "arg-extra-value";
const ARG_PREFIX: &str = "arg-prefix";
const ARG_SUFFIX: &str = "arg-suffix";
const ARG_SHA_LEN: &str = "arg-sha-len";

const ENV_SEED_PASSWORD: &str = "SEED_PASSWD";

const MIN_SHA_LEN: u64 = 16;
const MAX_SHA_LEN: u64 = 128;

/// Holds all command line arguments
#[derive(Clone, Debug)]
pub struct CliArguments {
    pub prefix: String,
    pub suffix: String,
    pub machine: String,
    pub account: String,
    pub seed: String,
    pub length: u64,
    pub extra: Option<Vec<String>>,
}

/// Parse the command line parameters with help of clap.
pub fn parse_cli_parameters() -> clap::ArgMatches {
    let min_len: &'static str = Box::leak(MIN_SHA_LEN.to_string().into_boxed_str());
    clap::Command::new(PROGRAM_NAME)
        .version(PROGRAM_VERSION)
        .author(PROGRAM_AUTHORS)
        .about(PROGRAM_DESCRIPTION)
        .arg(
            Arg::new(ARG_PREFIX)
                .long("prefix")
                .value_name("PREFIX")
                .help("Prefix that goes in front of the create password.")
                .default_value(DEFAULT_PREFIX)
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new(ARG_SUFFIX)
                .long("suffix")
                .value_name("SUFFIX")
                .help("Suffix that goes at the end of the create password.")
                .default_value(DEFAULT_SUFFIX)
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new(ARG_MACHINE_NAME)
                .long("machine")
                .value_name("MACHINE_NAME")
                .help("name of the machine that the password is created for.")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new(ARG_ACCOUNT_NAME)
                .long("account")
                .value_name("ACCOUNT_NAME")
                .help("name of the account that the password is created for.")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new(ARG_SEED_PASSWORD)
                .long("seed")
                .value_name("SEED_PASSWORD")
                .help(format!("seed password that is used to create a unique password. If not present the environment variable {} will be read. If that is empty, the user will be promped for a seed password.", ENV_SEED_PASSWORD))
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new(ARG_SHA_LEN)
                .long("length")
                .value_name("LENGTH")
                .help("Length of the sha portion of the created password.")
                .num_args(1)
                .default_value(min_len)
                .value_parser(value_parser!(u64).range(MIN_SHA_LEN..=MAX_SHA_LEN))
                .required(false),
        )
        .arg(
            Arg::new(ARG_EXTRA_VALUE)
                .long("extra")
                .value_name("EXTRA_VALUE")
                .help("Extra value(s) that should be used to create a password.")
                .action(clap::ArgAction::Append)
                .required(false),
        )
        .after_help(r##""##,)
        .get_matches()
}

/// Parse the command line parameters with help of clap and
/// return them in the `CliArguments` struct.
pub fn get_config() -> Result<CliArguments, Box<dyn Error>> {
    // parse cli parameters and load the configuration
    let clap_arg_matches = parse_cli_parameters();

    let seed_password: String = match clap_arg_matches.contains_id(ARG_SEED_PASSWORD) {
        true => clap_arg_matches
            .get_one::<String>(ARG_SEED_PASSWORD)
            .map(|f| f.to_string())
            .ok_or("Cannot read seed passsword")?,
        false => {
            let env_seed_password = std::env::var(ENV_SEED_PASSWORD);
            match env_seed_password {
                Ok(p) => p,
                Err(_) => rpassword::prompt_password("Enter seed password: ")?,
            }
        }
    };

    let extra: Option<Vec<String>> = if clap_arg_matches.contains_id(ARG_EXTRA_VALUE) {
        Some({
            let this = clap_arg_matches
                .get_many::<String>(ARG_EXTRA_VALUE)
                .ok_or("Cannot read extra value")?
                .map(|n| n.to_string());
            FromIterator::from_iter(this)
        })
    } else {
        None
    };

    Ok(CliArguments {
        prefix: clap_arg_matches
            .get_one::<String>(ARG_PREFIX)
            .map(|f| f.to_string())
            .ok_or("Cannot read prefix")?,
        suffix: clap_arg_matches
            .get_one::<String>(ARG_SUFFIX)
            .map(|f| f.to_string())
            .ok_or("Cannot read suffix")?,
        machine: clap_arg_matches
            .get_one::<String>(ARG_MACHINE_NAME)
            .map(|f| f.to_string())
            .ok_or("Cannot read machine name")?,
        account: clap_arg_matches
            .get_one::<String>(ARG_ACCOUNT_NAME)
            .map(|f| f.to_string())
            .ok_or("Cannot account name")?,
        seed: seed_password,
        length: *clap_arg_matches.get_one::<u64>(ARG_SHA_LEN).ok_or("Cannot parse length")?,
        extra,
    })
}
