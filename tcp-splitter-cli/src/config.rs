use std::net::SocketAddr;

use clap::{App, Arg, ArgMatches};

pub struct Config {
    listen_addr: SocketAddr,
    proxied_addr: SocketAddr,
    sniffer_addrs: Vec<SocketAddr>,
}

impl Config {
    pub fn listen_addr(&self) -> &SocketAddr {
        &self.listen_addr
    }

    pub fn proxied_addr(&self) -> &SocketAddr {
        &self.proxied_addr
    }

    pub fn sniffer_addrs(&self) -> &Vec<SocketAddr> {
        &self.sniffer_addrs
    }
}

impl Config {
    pub fn new_from_args() -> Config {
        let listen_addr_name = "listen address";
        let proxied_server_addr_name = "proxied server address";
        let sniffer_server_addr_name = "sniffer server address";

        let matches = App::new("tcp-splitter")
            .version(&format!("v{}", crate_version!())[..])
            .author("Bence SZIGETI <bence.szigeti@gohyda.com>")
            .about(crate_description!())
            .args(&[
                Arg::with_name(listen_addr_name)
                    .help("Listen address")
                    .long("listen")
                    .required(true)
                    .short("l")
                    .takes_value(true)
                    .validator(Config::is_addr),
                Arg::with_name(proxied_server_addr_name)
                    .help("Proxied channels: rx/tx")
                    .long("proxied")
                    .required(true)
                    .short("p")
                    .takes_value(true)
                    .validator(Config::is_addr),
                Arg::with_name(sniffer_server_addr_name)
                    .help("Proxied channels: rx")
                    .long("sniffer")
                    .multiple(true)
                    .required(false)
                    .short("s")
                    .takes_value(true)
                    .validator(Config::is_addr),
            ])
            .get_matches();

        Config {
            listen_addr: Config::get_addr(&matches, listen_addr_name),
            proxied_addr: Config::get_addr(&matches, proxied_server_addr_name),
            sniffer_addrs: Config::get_addr_list(&matches, sniffer_server_addr_name),
        }
    }

    fn get_addr(matches: &ArgMatches, cfg_name: &str) -> SocketAddr {
        let addr = matches.value_of(cfg_name).unwrap();
        addr.parse::<SocketAddr>().unwrap()
    }

    fn get_addr_list(matches: &ArgMatches, cfg_name: &str) -> Vec<SocketAddr> {
        if let Some(addrs) = matches.values_of(cfg_name) {
            return addrs
                .map(|addr| addr.parse::<SocketAddr>().unwrap())
                .collect();
        }
        Vec::new()
    }

    fn is_addr(addr: String) -> Result<(), String> {
        match addr.parse::<SocketAddr>() {
            Ok(_) => Ok(()),
            Err(_) => Err(addr),
        }
    }
}
