use std::{io::{stdin, stdout, Write, Error}, process::{Command, Stdio, Child}, env, path::Path};

fn main() {
    loop {
        print!("> ");
        if let Err(e) = stdout().flush() {
            handle_error(e);
        }

        let mut input = String::new();
        stdin().read_line(&mut input)
            .unwrap();

        let mut commands = input.trim()
            .split(" | ")
            .peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim()
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
                                    values.push(String::from(""));
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
                "export" => {
                    let mut args = args;
                    let key = args.next().map_or("", |x| x);
                    let value = args.next().map_or("", |x| x);
    
                    if key == "" {
                        for (key, value) in env::vars() {
                            println!("{}={}", key, value);
                        }
                    } else {
                        env::set_var(key, value);
                    }
                }
                command => {
                    let stdin = previous_command.map_or(
                        Stdio::inherit(), 
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            };
        }

        if let Some(mut final_command) = previous_command {
            if let Err(e) = final_command.wait() {
                handle_error(e);
            }
        }
    }
}

fn handle_error(e: Error) {
    eprintln!("{}", e);
}
