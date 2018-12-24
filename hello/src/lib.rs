use std::{
    fs,
    io::{prelude::*, Result},
    net::TcpStream,
    time::Duration,
};

pub fn handle_connections(mut stream: TcpStream) -> Result<()> {
    let mut buffer = vec![0; 512];

    // println!("buffer: {:#?}", buffer);
    let bytes_read = stream.read(&mut buffer)?;
    println!("Bytes that were read: {}", bytes_read);

    let contents = fs::read_to_string("hello.html")?;

    let response = format!("HTTP/1.1 200 OK \r\n\r\n{}", contents);
    stream.set_write_timeout(Some(Duration::new(5, 0)))?;

    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
