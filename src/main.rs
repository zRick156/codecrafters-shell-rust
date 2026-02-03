#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
       print!("$ ");
       io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command: Vec<&str> = command.split_whitespace().collect();
        match command[0] {
            "exit" => std::process::exit(0),
            "echo" => println!("{}", command[1..].join(" ")),
            _ => println!("{}: command not found", command[0]),
        }
    }
}

