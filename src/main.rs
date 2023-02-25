use std::{io::{stdin, stdout, Write, Error}, process::Command};

fn main() {
    loop {
        print!("> ");
        if let Err(e) = stdout().flush() {
            handle_error(e);
        }

        let mut input = String::new();
        stdin().read_line(&mut input)
            .unwrap();

        let mut parts = input.trim()
            .split_whitespace();
        let command = parts.next()
            .unwrap();
        let args = parts;

        let child = Command::new(command)
            .args(args)
            .spawn();
        
        match child {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    handle_error(e);
                }
            },
            Err(e) => handle_error(e)
        };
    }
}

fn handle_error(e: Error) {
    eprintln!("{}", e);
}
