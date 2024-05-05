use std::net::TcpStream;
use std::io::stdin;
use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;
use std::thread;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:80")?;

    // message reciving thread
    let stream_clone = stream.try_clone().unwrap();

    thread::spawn(move || {
        let stream = stream_clone;
        
        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        while reader.read_line(&mut buffer).unwrap() > 0 {
            println!("TEST");
            let message = buffer.trim_end().to_string();
            println!("{:?}", message);
            buffer.clear();
        }
    });

    // message sending loop
    loop {
        let mut buffer = String::new();
        let mut handle = stdin();

        handle.read_line(&mut buffer)?;

        stream.write(buffer.as_bytes())?;
    }

    Ok(())
}
