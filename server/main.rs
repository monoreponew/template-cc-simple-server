use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    const HOST : &str ="0.0.0.0";
    const PORT : &str ="8080";
    let endpoint : String = HOST.to_owned() + ":" +  PORT;
    let listener = TcpListener::bind(endpoint).unwrap();
    println!("listening on port {}",PORT);
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response_contents = String::from("Hello, world!");
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_contents.len(),
        response_contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
