use env_logger::Builder;

use tcp_splitter_cli::config::Config;
use tcp_splitter_cli::*;

fn main() {
    Builder::from_env("LOG").init();

    disallow_root();
    let cfg = Config::new_from_args();
    log_header();
    log_config(&cfg);

    run(cfg);
}
