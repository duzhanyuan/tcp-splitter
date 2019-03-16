use log::*;
use std::net::Shutdown;
use std::net::SocketAddr;

use futures::sync::mpsc::Receiver;
use tokio::net::TcpStream;

use crate::proxy::proxy_logger::ProxyLogger;
use crate::proxy::proxy_state::ProxyState;
use crate::SharedBuffer;

use super::upstream::upstream;

pub async fn handle_sniffer(
    sniffer_addr: SocketAddr,
    proxy_state: ProxyState,
    rx: Receiver<SharedBuffer>,
) {
    let sniffer = match await!(TcpStream::connect(&sniffer_addr)) {
        Ok(s) => {
            proxy_state.log_connected_to_sniffer_server(sniffer_addr);
            s
        }
        Err(_) => {
            proxy_state.log_failed_to_connected_to_sniffer_server(sniffer_addr);
            return;
        }
    };
    if sniffer.shutdown(Shutdown::Read).is_err() {
        warn!("Failed to close read stream for sniffer: {}", sniffer_addr);
    }
    if await!(upstream(sniffer, rx)).is_err() {
        proxy_state.log_sniffer_disconnected_too_early(sniffer_addr);
    }
    proxy_state.log_sniffer_disconnected(sniffer_addr);
}
