use std::net::SocketAddr;

use log::*;

use super::proxy_state::ProxyState;

pub trait ProxyLogger {
    fn log_client_connected(&self);
    fn log_client_disconnected(&self);
    fn log_connected_to_proxied_server(&self);
    fn log_connected_to_sniffer_server(&self, sniffer_addr: SocketAddr);
    fn log_failed_to_connect_to_proxied_server(&self);
    fn log_failed_to_connected_to_sniffer_server(&self, sniffer_addr: SocketAddr);
    fn log_proxied_disconnected(&self);
    fn log_sniffer_disconnected(&self, sniffer_addr: SocketAddr);
    fn log_sniffer_disconnected_too_early(&self, sniffer_addr: SocketAddr);
}

macro_rules! proxy_log_format {() => ("{event:<21} {client:^21} {arrow1:^5} {listener:^21} {arrow2:<5} {target:^21} [#{id}] {comment}")}

impl ProxyLogger for ProxyState {
    fn log_client_connected(&self) {
        info!(
            proxy_log_format!(),
            event = "Client connected:",
            client = format!("{}", self.client_addr()),
            arrow1 = "<--->",
            listener = format!("{}", self.server().listen_addr()),
            arrow2 = "",
            target = "",
            id = self.id(),
            comment = ""
        );
    }

    fn log_client_disconnected(&self) {
        info!(
            proxy_log_format!(),
            event = "Client disconnected:",
            client = format!("{}", self.client_addr()),
            arrow1 = "<-x->",
            listener = format!("{}", self.server().listen_addr()),
            arrow2 = "",
            target = "",
            id = self.id(),
            comment = ""
        );
    }

    fn log_connected_to_proxied_server(&self) {
        info!(
            proxy_log_format!(),
            event = "Proxy opened:",
            client = format!("{}", self.client_addr()),
            arrow1 = "<--->",
            listener = format!("{}", self.server().listen_addr()),
            arrow2 = "<--->",
            target = format!("{}", self.server().proxied_addr()),
            id = self.id(),
            comment = ""
        );
    }

    fn log_failed_to_connect_to_proxied_server(&self) {
        error!(
            proxy_log_format!(),
            event = "Proxy failed:",
            client = format!("{}", self.client_addr()),
            arrow1 = "<--->",
            listener = format!("{}", self.server().listen_addr()),
            arrow2 = "<-x->",
            target = format!("{}", self.server().proxied_addr()),
            id = self.id(),
            comment = "(failed to connect to proxied server)"
        );
    }

    fn log_connected_to_sniffer_server(&self, sniffer_addr: SocketAddr) {
        info!(
            proxy_log_format!(),
            event = "Sniffer proxy opened:",
            client = format!("{}", self.client_addr()),
            arrow1 = "<--->",
            listener = format!("{}", self.server().listen_addr()),
            arrow2 = "---->",
            target = format!("{}", sniffer_addr),
            id = self.id(),
            comment = ""
        );
    }

    fn log_failed_to_connected_to_sniffer_server(&self, sniffer_addr: SocketAddr) {
        warn!(
            proxy_log_format!(),
            event = "Sniffer proxy failed:",
            client = format!("{}", self.client_addr()),
            arrow1 = "<--->",
            listener = format!("{}", self.server().listen_addr()),
            arrow2 = "--x->",
            target = format!("{}", sniffer_addr),
            id = self.id(),
            comment = "(failed to connect to sniffer server)"
        );
    }

    fn log_sniffer_disconnected(&self, sniffer_addr: SocketAddr) {
        info!(
            proxy_log_format!(),
            event = "Sniffer proxy closed:",
            client = format!("{}", self.client_addr()),
            arrow1 = "<-x->",
            listener = format!("{}", self.server().listen_addr()),
            arrow2 = "--x->",
            target = format!("{}", sniffer_addr),
            id = self.id(),
            comment = ""
        );
    }

    fn log_proxied_disconnected(&self) {
        info!(
            proxy_log_format!(),
            event = "Proxy closed:",
            client = format!("{}", self.client_addr()),
            arrow1 = "<-x->",
            listener = format!("{}", self.server().listen_addr()),
            arrow2 = "<-x->",
            target = format!("{}", self.server().proxied_addr()),
            id = self.id(),
            comment = ""
        );
    }

    fn log_sniffer_disconnected_too_early(&self, sniffer_addr: SocketAddr) {
        warn!(
            proxy_log_format!(),
            event = "Sniffer proxy closed:",
            client = format!("{}", self.client_addr()),
            arrow1 = "<--->",
            listener = format!("{}", self.server().listen_addr()),
            arrow2 = "<-x->",
            target = format!("{}", sniffer_addr),
            id = self.id(),
            comment = "(sniffer disconnected too early)"
        );
    }
}
