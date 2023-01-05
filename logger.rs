use std::io::{self, Read, Write};
use std::net::{TcpStream, TcpListener};
use std::{thread, time};
use ini::Ini;

fn main() {
    
    let config = Ini::load_from_file("config.ini").unwrap();
    let parent_ip = config.section(Some("Parent")).unwrap().get("IP").unwrap();
    
    let mut stream = TcpStream::connect(format!("{}:8080", parent_ip)).unwrap();

    loop {
        
        let mut keystroke = [0];
        io::stdin().read_exact(&mut keystroke).unwrap();

        stream.write_all(&keystroke).unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}
