use std::io;

use futures::sync::mpsc::Receiver;
use tokio::io::shutdown;
use tokio::prelude::*;

use crate::proxy::proxy_stream::ProxyStream;
use crate::SharedBuffer;

pub async fn upstream(mut proxied: ProxyStream, mut rx: Receiver<SharedBuffer>) -> io::Result<()> {
    while let Some(data) = await!(rx.next()) {
        let (data, n) = *data.unwrap();
        if n == 0 {
            await!(shutdown(proxied))?;
            break;
        }
        await!(proxied.write_all_async(&data[0..n]))?;
    }
    Ok(())
}
