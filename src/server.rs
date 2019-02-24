use rand::{prelude::ThreadRng, thread_rng};
use std::collections::HashSet;
use std::io;
use std::net::{ToSocketAddrs, SocketAddr, SocketAddrV4, UdpSocket};

use crate::protocol::{
    AnnounceRequest, ConnectionID, ConnectResponse, ConnectRequest, Request,
    ScrapeRequest, Writable
};


/// Represents the information associated with the torrent
struct TorrentInfo {
    leechers: i32,
    seeders: i32,
    peers: HashSet<SocketAddrV4>
}

impl TorrentInfo {
    /// Add a new peer to an existing torrent
    fn add_new_peer(&mut self, peer: SocketAddr) {
        match peer {
            SocketAddr::V4(ip) => { 
                if self.peers.insert(ip) {
                    self.leechers += 1;
                }
            }
            // We don't handle v6 address
            SocketAddr::V6(_) => {}
        }
    }

    /// Create a torrent from the first peer to announce it
    fn from_first_peer(peer: SocketAddr) -> Self {
        let mut info = TorrentInfo {
            leechers: 0, seeders: 0, peers: HashSet::new()
        };
        match peer {
            SocketAddr::V4(ip) => {
                info.peers.insert(ip);
                info.seeders += 1;
            }
            SocketAddr::V6(_) => {}
        }
        info
    }
    
    fn sample_peers(&self) -> Vec<SocketAddrV4> {
        let mut acc = Vec::with_capacity(self.peers.len());
        for &p in &self.peers {
            acc.push(p);
        }
        acc
    }
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
                self.handle_request(src, &r)?;
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
            Request::Connect(r) => self.handle_connect(src, r),
            Request::Announce(r) => self.handle_announce(src, r),
            Request::Scrape(r) => self.handle_scrape(src, r)
        }
    }

    fn handle_connect(&mut self, src: SocketAddr, req: &ConnectRequest) -> io::Result<()> {
        // We do nothing if the magic id is wrong
        if req.connection_id.is_magic_id() {
            let connection_id = ConnectionID::random(&mut self.rng);
            let transaction_id = req.transaction_id;
            let response = ConnectResponse {
                transaction_id, connection_id
            };
            self.write_to_socket(response, src)?;
        }
        Ok(())
    }

    fn handle_announce(&mut self, src: SocketAddr, req: &AnnounceRequest) -> io::Result<()> {
        Ok(())
    }

    fn handle_scrape(&mut self, src: SocketAddr, req: &ScrapeRequest) -> io::Result<()> {
        Ok(())
    }
}
