use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buff = [0; 512];
        stream.read(&mut buff)?;
        stream.write(b"Hello from server")?;
    }

    Ok(())
}
