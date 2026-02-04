use pathsearch::find_executable_in_path;
use shell_words::split;
#[allow(unused_imports)]
use std::io::{self, Write};

const BUILT_IN_COMMANDS: [&str; 3] = ["type", "exit", "echo"];
enum Command {
    ExitCommand,
    EchoCommand { display_string: String },
    TypeCommand { command_name: String },
    TypeExternalProgram { command_name: String, args: Vec<String> },
    CommandNotFound,
}

impl Command {
    fn from_input(input: &str) -> Self {
        let input = input.trim();
        if input.is_empty() {
            return Self::CommandNotFound;
        }

        let parts = match split(input) {
            Ok(p) if !p.is_empty() => p,
            _ => return Self::CommandNotFound,
        };

        match parts[0].as_str() {
            "exit" => return Self::ExitCommand,
            "echo" => {
                let display = if parts.len() > 1 {
                    parts[1..].join(" ")
                } else {
                    String::new()
                };
                return Self::EchoCommand { display_string: display };
            }
            "type" => {
                if parts.len() > 1 {
                    return Self::TypeCommand {
                        command_name: parts[1].clone(),
                    };
                } else {
                    return Self::CommandNotFound;
                }
            }
            _ => {}
        }

        // Cerca l'eseguibile usando il primo token (gestisce anche percorsi quotati)
        if let Some(path) = find_executable_in_path(&parts[0]) {
            return Self::TypeExternalProgram {
                command_name: parts[0].clone(),
                args: if parts.len() > 1 {
                    parts[1..].to_vec()
                } else {
                    Vec::new()
                },
            };
        }

        Self::CommandNotFound
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim();
        command_handler(trimmed_input);
    }
}

fn command_handler(input: &str) {
    if input.is_empty() {
        return;
    }

    let command = Command::from_input(input);

    match command {
        Command::ExitCommand => std::process::exit(0),
        Command::EchoCommand { display_string } => println!("{}", display_string),
        Command::TypeCommand { command_name } => {
            if BUILT_IN_COMMANDS.contains(&command_name.as_str()) {
                println!("{} is a shell builtin", command_name);
            } else if let Some(path) = find_executable_in_path(&command_name) {
                println!("{} is {}", command_name, path.display());
            } else {
                println!("{}: not found", command_name);
            }
        }
        Command::TypeExternalProgram { command_name, args } => {
            match std::process::Command::new(&command_name).args(&args).status() {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("process exited with code {:?}", status.code());
                    }
                }
                Err(e) => eprintln!("failed to execute process: {}", e),
            }
        }
        _ => println!("{}: command not found", input.trim()),
    }
}