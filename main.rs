use std::io::{self, Write};

fn main() {
    println!("iLonhro Terminal");
    println!("Type 'help' for commands, 'exit' to quit.");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("ilonhro> ");
        let _ = stdout.flush();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            eprintln!("Failed to read input.");
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        match trimmed {
            "exit" | "quit" => break,
            "help" => {
                println!("Commands: help, exit, quit");
                println!("Shell execution is not implemented yet.");
            }
            _ => {
                println!("Unrecognized command: {trimmed}");
            }
        }
    }
}
