use std::io::{Read, Write};
use std::net::TcpListener;

struct Server;

impl Server {
    fn start(port: &str) -> std::io::Result<()> {
        let mut pattern = "127.0.0.1:".to_string();
        pattern.push_str(&port);

        let listener = TcpListener::bind(pattern)?;

        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buff = [0; 512];
            stream.read(&mut buff)?;
            stream.write(b"Hello from server")?;
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    Server::start("8080")?;

    Ok(())
}
