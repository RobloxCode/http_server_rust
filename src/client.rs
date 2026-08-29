use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.write(b"Hello from the client")?;

    let mut buff = [0; 512];
    stream.read(&mut buff)?;

    println!("Recieved: {}", String::from_utf8_lossy(&buff));

    Ok(())
}
