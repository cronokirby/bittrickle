use rand::{prelude::ThreadRng, thread_rng};
use std::io;
use std::net::{ToSocketAddrs, SocketAddr, UdpSocket};

use crate::protocol::{ConnectionID, ConnectResponse, Request, Writable};


fn handle_request(src: SocketAddr, request: &Request) {
    println!("New request from {:?}", src);
    println!("{:?}", request);
}

/// Holds all the state a server needs to run
pub struct Server {
    rng: ThreadRng,
    socket: UdpSocket,
    read_buf: Vec<u8>,
    write_buf: Vec<u8>
}

impl Server {
    /// Create a new server, with an address to bind the socket to.
    /// The socket might not be able to be created, so this
    /// function returns an io result.
    pub fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
        let rng = thread_rng();
        let socket = UdpSocket::bind(addr)?;
        let read_buf = vec![0; 2048];
        let write_buf = vec![0; 2048];
        Ok(Server { rng, socket, read_buf, write_buf })
    }

    /// Run the server, blocking the current thread
    /// If an io error occurrs at any point, this function returns.
    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            let (amt, src) = self.socket.recv_from(&mut self.read_buf)?;
            let request = Request::from_bytes(&self.read_buf[..amt]);
            if let Ok(r) = request {
                self.handle_request(src, &r);
            }
        }
    }
 
    fn write_to_socket(&mut self, w: impl Writable, src: SocketAddr) -> io::Result<()> {
        let count = w.write(&mut self.write_buf);
        let mut start = 0;
        while start < count {
            let slice = &self.write_buf[start..count];
            start = self.socket.send_to(slice, src)?;
        }
        Ok(())
    }
    
    fn handle_request(&mut self, src: SocketAddr, request: &Request) -> io::Result<()> {
        match request {
            Request::Connect(r) => {
                // We do nothing if the magic id is wrong
                if r.connection_id.is_magic_id() {
                    let connection_id = ConnectionID::random(&mut self.rng);
                    let transaction_id = r.transaction_id;
                    let response = ConnectResponse {
                        transaction_id, connection_id
                    };
                    self.write_to_socket(response, src)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
