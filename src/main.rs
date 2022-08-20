use std::env;

use pretty_env_logger;

use checkrs::cli;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init_timed();

    cli::run();
}
