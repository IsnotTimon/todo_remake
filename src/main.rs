use std::io::{self, Write};

use clap::Parser;
use cli::{Cli, Commands};
use database::Database;

#[allow(dead_code)]

mod database;
mod db_operation;
mod cli;
mod commands;

fn main() {
    println!("Hello, user!");
    let mut _db = Database::open_database_file();
    
    println!("Welcome to the Todo CLI! Type 'exit' to quit.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input. Please try again.");
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        let args = match Cli::try_parse_from(std::iter::once("todo").chain(input.split_whitespace())) {
            Ok(parsed) => parsed,
            Err(err) => {
                eprintln!("\x1b[31merror parsing command:\x1b[39m {}", err);
                continue;
            }
        };
        
        let result = match args.command {
            Commands::Info => commands::info(),
            Commands::Remove { id } => commands::remove(),
            _ => commands::info(),
        };
    
        if let Err(err) = result {
            eprintln!(" {} ",err);
            std::process::exit(1);
        }
    }

}
