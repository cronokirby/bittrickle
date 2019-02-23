use rand::{prelude::ThreadRng, thread_rng};
use std::net::{ToSocketAddrs, SocketAddr, UdpSocket};

use crate::protocol::Request;


fn handle_request(src: SocketAddr, request: &Request) {
    println!("New request from {:?}", src);
    println!("{:?}", request);
}

/// Holds all the state a server needs to run
pub struct Server {
    rng: ThreadRng,
    socket: UdpSocket,
    read_buf: Vec<u8>
}

impl Server {
    /// Create a new server, with an address to bind the socket to.
    /// The socket might not be able to be created, so this
    /// function returns an io result.
    pub fn new(addr: impl ToSocketAddrs) -> std::io::Result<Self> {
        let rng = thread_rng();
        let socket = UdpSocket::bind(addr)?;
        let read_buf = vec![0; 2048];
        Ok(Server { rng, socket, read_buf })
    }

    /// Run the server, blocking the current thread
    /// If an io error occurrs at any point, this function returns.
    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            let (amt, src) = self.socket.recv_from(&mut self.read_buf)?;
            let request = Request::from_bytes(&self.read_buf[..amt]);
            if let Ok(r) = request {
                handle_request(src, &r);
            }
        }
    }
}
