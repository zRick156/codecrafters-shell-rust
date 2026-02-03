#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    let mut command: String = String::new();
    while !command.contains("exit") {
       print!("$ ");
       io::stdout().flush().unwrap();

        io::stdin().read_line(&mut command).unwrap();
        if command.contains("exit") {
            println!("{}: command not found", command.trim());
        }
    }
}
