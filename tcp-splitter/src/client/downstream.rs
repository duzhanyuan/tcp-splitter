use std::io;

use tokio::io::shutdown;
use tokio::prelude::*;

use crate::proxy::proxy_stream::ProxyStream;
use crate::BUFFER_SIZE;

pub async fn downstream(mut client: ProxyStream, mut proxied: ProxyStream) -> io::Result<()> {
    let mut buf = [0; BUFFER_SIZE];
    loop {
        match await!(proxied.read_async(&mut buf))? {
            0 => {
                await!(shutdown(client))?;
                break;
            }
            n => {
                await!(client.write_all_async(&buf[0..n]))?;
            }
        }
    }
    Ok(())
}
