use std::io;
use std::sync::Arc;

use futures::sync::mpsc;
use log::*;
use tokio::prelude::*;

use crate::proxy::proxy_stream::ProxyStream;
use crate::SnifferTxs;
use crate::BUFFER_SIZE;

use super::upstream::upstream;

pub async fn splitted_upstream(
    mut client: ProxyStream,
    proxied: ProxyStream,
    sniffer_txs: SnifferTxs,
) -> io::Result<()> {
    let (tx, rx) = mpsc::channel(10);
    tokio::spawn_async(
        async move {
            if let Err(e) = await!(upstream(proxied, rx)) {
                trace!("Client downstream broken: {:?}", e);
            };
        },
    );

    let mut buf = [0; BUFFER_SIZE];
    loop {
        match await!(client.read_async(&mut buf))? {
            n => {
                let buf = Arc::new((buf, n));

                if let Err(e) = await!(tx.clone().send(buf.clone())) {
                    trace!("Channel already gone: {:?}", e);
                }
                let sniffer_txs = sniffer_txs.lock().unwrap();
                for (_, tx) in sniffer_txs.iter() {
                    if let Err(e) = tx.clone().try_send(buf.clone()) {
                        trace!("Channel already gone: {:?}", e);
                    }
                }
                if n == 0 {
                    break;
                }
            }
        }
    }
    Ok(())
}
