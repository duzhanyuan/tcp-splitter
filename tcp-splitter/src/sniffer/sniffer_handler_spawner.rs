use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::sync::mpsc;

use crate::proxy::proxy_state::ProxyState;
use crate::SnifferTxs;

use super::sniffer_handler::handle_sniffer;

pub fn spawn_sniffer_handlers(proxy_state: ProxyState) -> SnifferTxs {
    let txs = Arc::new(Mutex::new(HashMap::with_capacity(
        proxy_state.server().sniffer_addrs().len(),
    )));

    for addr in proxy_state.server().sniffer_addrs() {
        let (tx, rx) = mpsc::channel(10);
        txs.lock().unwrap().insert(addr.clone(), tx);
        let txs = Arc::downgrade(&txs);
        let addr = *addr;
        let proxy_state = proxy_state.clone();
        tokio::spawn_async(
            async move {
                await!(handle_sniffer(addr, proxy_state, rx));
                if let Some(txs) = txs.upgrade() {
                    txs.lock().unwrap().remove(&addr);
                }
            },
        );
    }

    txs
}
