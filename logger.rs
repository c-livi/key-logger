use std::{env, io::{self, Write}, process::Command};

fn main() {
    let mut buffer = String::new();

    loop {
        let mut input = String::new();
        let mut output = String::new();

        io::stdin().read_line(&mut input).unwrap();
        
        buffer.push_str(&input);
        
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        let os_status = Command::new("systemctl")
            .arg("is-system-running")
            .output()
            .unwrap();

        if !os_status.status.success() {
            break;
        }
    }

    let username = env::var("USER").unwrap();
    let mut file = String::new();
    file.push_str("/home/");
    file.push_str(&username);
    file.push_str("/log.txt");
    std::fs::write(file, buffer).unwrap();
}
