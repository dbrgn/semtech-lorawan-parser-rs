extern crate futures;
#[macro_use]
extern crate tokio_core;

use std::io;
use std::net::SocketAddr;

use futures::{Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            let count: usize = try_nb!(self.socket.recv_from(&mut self.buf)).0;
            println!("Received {} bytes", count);
            self.process_packet(&self.buf[..count]);
        }
    }
}

impl Server {
    fn process_packet(&self, bytes: &[u8]) {
        println!("Processing packet...");
    }
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    let mut eventloop = Core::new().unwrap();
    let handle = eventloop.handle();
    let socket = UdpSocket::bind(&addr, &handle).unwrap();
    println!("Listening on: {}", addr);

    eventloop.run(Server {
        socket: socket,
        buf: vec![0; 1024],
    }).unwrap();
}
