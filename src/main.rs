use once_cell::sync::Lazy;
use pathsearch::find_executable_in_path;
use shell_words::split;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::sync::Mutex;

const BUILT_IN_COMMANDS: [&str; 5] = ["type", "exit", "echo", "pwd", "cd"];

static CURRENT_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(initial_current_dir()));

fn initial_current_dir() -> String {
    match std::env::current_dir() {
        Ok(p) => p.display().to_string(),
        Err(_) => String::from("unknown"),
    }
}

enum Command {
    ExitCommand,
    EchoCommand {
        display_string: String,
    },
    TypeCommand {
        command_name: String,
    },
    ExecCommand {
        command_name: String,
        args: Vec<String>,
    },
    PwdCommand {
        path_name: String,
    },
    CommandNotFound,
    CdCommand {
        path_name: String,
    },
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
                return Self::EchoCommand {
                    display_string: display,
                };
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
            "pwd" => {
                return Self::PwdCommand {
                    path_name: CURRENT_PATH.lock().unwrap().clone(),
                };
            }
            "cd" => {
                let path = if parts.len() > 1 {
                    parts[1].clone()
                } else {
                    CURRENT_PATH.lock().unwrap().clone()
                };
                return Self::CdCommand { path_name: path };
            }
            _ => {}
        }

        if find_executable_in_path(&parts[0]).is_some() {
            return Self::ExecCommand {
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
        let bytes_read = stdin.read_line(&mut input).unwrap();
        if bytes_read == 0 {
            break;
        }
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
        Command::ExecCommand { command_name, args } => {
            match std::process::Command::new(&command_name)
                .args(&args)
                .status()
            {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("process exited with code {:?}", status.code());
                    }
                }
                Err(e) => eprintln!("failed to execute process: {}", e),
            }
        }
        Command::PwdCommand { path_name } => {
            println!("{}", path_name);
        }
        Command::CdCommand { path_name } => {
            let expanded = if path_name.starts_with("~") {
                let home = std::env::var("HOME")
                    .or_else(|_| std::env::var("USERPROFILE"))
                    .unwrap_or_default();
                if path_name == "~" {
                    home
                } else {
                    format!("{}{}", home, &path_name[1..])
                }
            } else {
                path_name.clone()
            };

            if let Err(_e) = std::env::set_current_dir(&expanded) {
                eprintln!("cd: {}: {}", path_name, "No such file or directory");
            } else {
                // aggiorna la variabile globale con il nuovo path effettivo
                let new_path = match std::env::current_dir() {
                    Ok(p) => p.display().to_string(),
                    Err(_) => expanded,
                };
                *CURRENT_PATH.lock().unwrap() = new_path;
            }
        }
        _ => println!("{}: command not found", input.trim()),
    }
}
