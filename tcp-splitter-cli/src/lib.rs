#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate clap;

use std::process::exit;

use config::Config;
use hostname::get_hostname;
use log::*;
use users::{get_current_uid, get_user_by_uid};

use tcp_splitter::run_async_tcp_splitter;

pub mod config;

pub fn logged_exit(status: i32) {
    info!("Exit");
    exit(status);
}

pub fn log_header() {
    let uid = get_current_uid();
    let user = get_user_by_uid(uid).unwrap();
    info!(
        "{} {} running on '{}' as '{}'",
        crate_name!(),
        crate_version!(),
        get_hostname().unwrap(),
        user.name().to_string_lossy()
    );
}

pub fn log_config(cfg: &Config) {
    info!("{0:<15} {1}", "Listener:", cfg.listen_addr());
    info!("{0:<15} {1}", "Proxied server:", cfg.proxied_addr());
    for addr in cfg.sniffer_addrs() {
        info!("{0: <15} {1}", "Sniffer server:", addr);
    }
}

pub fn disallow_root() {
    let uid = get_current_uid();
    if uid == 0 {
        error!("Do not run as root");
        logged_exit(1);
    }
}

pub fn run(cfg: Config) {
    tokio::run_async(
        async move {
            match await!(run_async_tcp_splitter(
                *cfg.listen_addr(),
                *cfg.proxied_addr(),
                cfg.sniffer_addrs().clone()
            )) {
                Ok(_) => logged_exit(0),
                Err(e) => {
                    error!("{}", e);
                    logged_exit(1);
                }
            };
        },
    );
}
