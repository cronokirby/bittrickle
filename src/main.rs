extern crate rand;

mod protocol;
use protocol::Request;

use std::net::{SocketAddr, UdpSocket};


fn handle_request(src: SocketAddr, request: &Request) {
    println!("New request from {:?}", src);
    println!("{:?}", request);
}

fn main() -> std::io::Result<()> {
    let mut socket = UdpSocket::bind("127.0.0.1:8080")?;
    let mut buffer = vec![0; 2048];
    loop {
        let (amt, src) = socket.recv_from(&mut buffer)?;
        let request = Request::from_bytes(&buffer[..amt]);
        request.map(|r| handle_request(src, &r));
    }
    Ok(())
}
