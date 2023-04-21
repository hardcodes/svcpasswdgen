pub mod base64_trait;
pub mod cli_parser;
pub mod password;

pub const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROGRAM_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PROGRAM_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub const ARGON2_MEMORY_COST: u32 = 65635;
pub const ARGON2_NUM_ITERATIONS: u32 = 10;
pub const ARGON2_NUM_PARALLEL_THREADS: u32 = 1;
pub const ARGON2_OUTPUT_LENGTH: usize = 32;