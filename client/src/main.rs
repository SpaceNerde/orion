use std::net::TcpStream;
use std::io::stdin;
use std::io::Write;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:80")?;

    // Active messaging loop

    loop {
        let mut buffer = String::new();
        let mut handle = stdin();

        handle.read_line(&mut buffer)?;

        stream.write(buffer.as_bytes())?;
    }

    Ok(())
}
