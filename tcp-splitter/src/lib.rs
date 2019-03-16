#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

mod client;
mod proxy;
mod server;
mod sniffer;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures::sync::mpsc;

use crate::server::acceptor::run_acceptor;

const BUFFER_SIZE: usize = 65535;

type SharedBuffer = Arc<([u8; BUFFER_SIZE], usize)>;
type SnifferTx = mpsc::UnboundedSender<SharedBuffer>;
type SnifferTxs = Arc<Mutex<HashMap<SocketAddr, SnifferTx>>>;

/// Run "tcp-splitter".
///
/// It's create a TCP proxy server with the ability to copy clients upstream to sniffer servers.
///
/// # Inputs
/// * `listen_addr` is a listen address for the proxy server.
/// * `proxied_addr` is a proxied server address. Incoming clients proxied to this server.
/// * `sniffer_addrs` is a list of sniffer addresses. Only Rx channel of incoming clients are redirected to this servers.
///
/// # Returns
/// The `Future` only completes with an error `String` when listen address bind is failed -- otherwise run forever.
///
/// ## TODO
/// * Graceful shutdown on signal (`SIGINT`/`SIGTERM`):
///   * stop listening,
///   * build up partly-initiated proxy channels,
///   * wait for ongoing proxy channels to finish;
///   * for repeated signal, force shutdown.
pub async fn run_async_tcp_splitter(
    listen_addr: SocketAddr,
    proxied_addr: SocketAddr,
    sniffer_addrs: Vec<SocketAddr>,
) -> Result<(), std::string::String> {
    await!(run_acceptor(listen_addr, proxied_addr, sniffer_addrs))
}
