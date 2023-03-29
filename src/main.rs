use svcpasswdgen::cli_parser::get_config;

fn main() {
    let cli_args = get_config();
    println!("cli_args = {:?}", &cli_args);
}
