use std::io;

use futures::sync::mpsc::Receiver;
use tokio::net::TcpStream;
use tokio::prelude::*;

use crate::SharedBuffer;

pub async fn upstream(mut sniffer: TcpStream, mut rx: Receiver<SharedBuffer>) -> io::Result<()> {
    while let Some(data) = await!(rx.next()) {
        let (data, n) = *data.unwrap();
        await!(sniffer.write_all_async(&data[0..n]))?;
    }
    Ok(())
}
