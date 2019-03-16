use std::net::SocketAddr;
use std::sync::Arc;

use log::*;
use tokio::net::TcpListener;
use tokio::prelude::*;

use crate::client::client_handler::handle_client;
use crate::proxy::proxy_state::ProxyServerEndpoints;
use crate::proxy::proxy_state::ProxyState;

fn bind(listen_addr: SocketAddr) -> Result<TcpListener, std::string::String> {
    let listener = match TcpListener::bind(&listen_addr) {
        Ok(s) => s,
        Err(_) => return Err(format!("Could not bind to {}", listen_addr)),
    };
    Ok(listener)
}

pub async fn run_acceptor(
    listen_addr: SocketAddr,
    proxied_addr: SocketAddr,
    sniffer_addrs: Vec<SocketAddr>,
) -> Result<(), std::string::String> {
    let listener = bind(listen_addr)?;
    let mut id = 0;
    let server_addrs = Arc::new(ProxyServerEndpoints::new(
        listen_addr,
        proxied_addr,
        sniffer_addrs,
    ));
    info!("{0:<21} {1}", "Started listening on:", listen_addr);

    let mut incoming = listener.incoming();
    while let Some(client) = await!(incoming.next()) {
        let client = match client {
            Ok(client) => client,
            Err(_) => continue,
        };
        let client_addr = match client.peer_addr() {
            Ok(client_addr) => client_addr,
            Err(_) => continue,
        };
        let proxy_state = ProxyState::new(id, client_addr, server_addrs.clone());
        id += 1;
        tokio::spawn_async(
            async move {
                if client.set_nodelay(true).is_err() {
                    warn!("Failed to disable TCP_NODELAY for {}", client_addr);
                }
                await!(handle_client(proxy_state, client));
            },
        );
    }

    info!("{0:<21} {1}", "Stopped listening on:", listen_addr);
    Ok(())
}
