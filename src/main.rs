extern crate rand;

mod protocol;
mod server;


fn main() -> std::io::Result<()> {
    let mut server = server::Server::new("127.0.0.1:8080")?;
    server.run()
}
