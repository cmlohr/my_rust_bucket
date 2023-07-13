use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // 1024 bytes buffer
    stream.read(&mut buffer).unwrap();
    

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();
    let parts: Vec<&str> = request_line.split(" ").collect();
    let method = parts[0];
    let path = parts[1];
    let version = parts[2];
    println!("{} {} {}", method, path, version);
    let responce = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
    stream.write(responce.as_bytes()).unwrap();
    stream.flush().unwrap();
}