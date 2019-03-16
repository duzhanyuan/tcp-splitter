use std::sync::{Arc, Mutex};

use log::*;
use tokio::net::TcpStream;

use crate::proxy::proxy_logger::ProxyLogger;
use crate::proxy::proxy_state::ProxyState;
use crate::proxy::proxy_stream::ProxyStream;
use crate::sniffer::sniffer_handler_spawner::spawn_sniffer_handlers;

use super::downstream::downstream;
use super::splitted_upstream::splitted_upstream;

pub async fn handle_client(proxy_state: ProxyState, client: TcpStream) {
    proxy_state.log_client_connected();
    let proxied = match await!(TcpStream::connect(&proxy_state.server().proxied_addr())) {
        Ok(s) => {
            proxy_state.log_connected_to_proxied_server();
            s
        }
        Err(_) => {
            proxy_state.log_failed_to_connect_to_proxied_server();
            return;
        }
    };
    let sniffer_txs = spawn_sniffer_handlers(proxy_state.clone());

    let client = ProxyStream(Arc::new(Mutex::new(client)));
    let proxied = ProxyStream(Arc::new(Mutex::new(proxied)));
    let client_copy = client.clone();
    let proxied_copy = proxied.clone();
    tokio::spawn_async(
        async move {
            if let Err(e) = await!(splitted_upstream(client_copy, proxied_copy, sniffer_txs)) {
                trace!("Upstream broken: {:?}", e);
            };
        },
    );
    if let Err(e) = await!(downstream(client, proxied)) {
        trace!("Downstream broken: {:?}", e);
    };

    proxy_state.log_client_disconnected();
    proxy_state.log_proxied_disconnected();
}
