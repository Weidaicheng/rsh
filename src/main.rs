use std::{io::{stdin, stdout, Write, Error}, process::Command, env, path::Path};

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

        match command {
            "cd" => {
                let new_dir = args.peekable()
                    .peek()
                    .map_or("/", |x| *x);
                let dest = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(dest) {
                    handle_error(e);
                }
            },
            "exit" => return,
            "echo" => {
                let mut values: Vec<String> = Vec::new();
                for arg in args {
                    if arg.starts_with("$") {
                        match env::var(arg.replace("$", "")) {
                            Ok(value) => {
                                values.push(value);
                            },
                            Err(_) => {
                                values.push(String::from(arg));
                            }
                        };
                    } else {
                        values.push(String::from(arg));
                    }
                }

                let child = Command::new(command)
                    .args(values)
                    .spawn();
                
                match child {
                    Ok(mut child) => {
                        if let Err(e) = child.wait() {
                            handle_error(e);
                        }
                    },
                    Err(e) => handle_error(e)
                };
            },
            command => {
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
    }
}

fn handle_error(e: Error) {
    eprintln!("{}", e);
}
