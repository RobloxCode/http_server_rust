use std::io::{Read, Write};
use std::net::TcpStream;

const BUFSIZE: usize = 512;

struct Client {
    msg_buf: [u8; BUFSIZE],
}

impl Client {
    fn request(port: &str) -> std::io::Result<()> {
        let mut pattern = "127.0.0.1:".to_string();
        pattern.push_str(&port);

        let mut stream = TcpStream::connect(&pattern)?;

        stream.write(b"Hello from the client")?;

        let mut buff = [0; BUFSIZE];
        stream.read(&mut buff)?;

        println!("Recieved: {}", String::from_utf8_lossy(&buff));

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    Client::request("8080")?;

    Ok(())
}
