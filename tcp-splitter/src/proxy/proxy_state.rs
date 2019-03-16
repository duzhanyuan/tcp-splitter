use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProxyState {
    id: usize,
    client_addr: SocketAddr,
    server_addr: Arc<ProxyServerEndpoints>,
}

impl ProxyState {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn client_addr(&self) -> &SocketAddr {
        &self.client_addr
    }

    pub fn server(&self) -> &Arc<ProxyServerEndpoints> {
        &self.server_addr
    }
}

impl ProxyState {
    pub fn new(
        id: usize,
        client_addr: SocketAddr,
        server_addr: Arc<ProxyServerEndpoints>,
    ) -> ProxyState {
        ProxyState {
            id,
            client_addr,
            server_addr,
        }
    }
}

pub struct ProxyServerEndpoints {
    listen_addr: SocketAddr,
    proxied_addr: SocketAddr,
    sniffer_addrs: Vec<SocketAddr>,
}

impl ProxyServerEndpoints {
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

impl ProxyServerEndpoints {
    pub fn new(
        listen_addr: SocketAddr,
        proxied_addr: SocketAddr,
        sniffer_addrs: Vec<SocketAddr>,
    ) -> ProxyServerEndpoints {
        ProxyServerEndpoints {
            listen_addr,
            proxied_addr,
            sniffer_addrs,
        }
    }
}
