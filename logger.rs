use std::{env, io::{self, Write}, process::Command};

fn main() {
    let mut buffer = String::new();

    loop {
        let mut input = String::new();
        let mut output = String::new();

        // Read input from the keyboard
        io::stdin().read_line(&mut input).unwrap();

        // Record the input to the buffer
        buffer.push_str(&input);

        // Execute the input as a command
        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(&input);
        let output = cmd.output().unwrap();

        // Write the output of the command to the screen
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        // Check if the command was "exit"
        if input.trim() == "exit" {
            break;
        }
    }

    // Write the logged keystrokes to a file
    let username = env::var("USER").unwrap();
    let mut file = String::new();
    file.push_str("/home/");
    file.push_str(&username);
    file.push_str("/log.txt");
    std::fs::write(file, buffer).unwrap();
}
