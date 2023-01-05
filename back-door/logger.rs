use std::io::{self, Write};
use std::net::TcpListener;
use std::fs;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();
        fs::write("src/log/log.txt", &buffer).unwrap();
    }
}
