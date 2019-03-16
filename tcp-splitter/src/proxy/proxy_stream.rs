use std::io;
use std::net::Shutdown;
use std::sync::{Arc, Mutex};

use futures::Poll;
use tokio::net::TcpStream;
use tokio::prelude::*;

#[derive(Clone)]
pub struct ProxyStream(pub Arc<Mutex<TcpStream>>);

impl Read for ProxyStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.lock().unwrap().read(buf)
    }
}

impl Write for ProxyStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        unreachable!();
    }
}

impl AsyncRead for ProxyStream {}

impl AsyncWrite for ProxyStream {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.0.lock().unwrap().shutdown(Shutdown::Write)?;
        Ok(().into())
    }
}
