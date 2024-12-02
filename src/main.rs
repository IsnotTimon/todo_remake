use std::io::{self, Write};
use clap::Parser;
use cli::{Cli, Commands};
use database::Database;

mod database;
mod db_operation;
mod cli;
mod commands;

fn main() {
    let mut db = Database::open_database_file().unwrap();

    // Use loop to run.
    println!("Welcome to the Todo CLI! Type 'exit' to quit.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // Read command line content.
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input. Please try again.");
            continue;
        }

        // Process the content or exit this APP.
        let input = input.trim();
        println!("GOT:{}",input);
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Parse the content to TODO_command.
        let args = match Cli::try_parse_from(std::iter::once("todo").chain(input.split_whitespace())) {
            Ok(parsed) => parsed,
            Err(err) => {
                eprintln!("\x1b[31merror parsing command:\x1b[39m {}", err);
                continue;
            }
        };

        // Execute the TODO_command.
        let result = match args.command {
            Commands::Info => commands::info(),
            Commands::Add { content } => commands::add(&mut db, content),
            Commands::Remove { id } => commands::remove(&mut db, id),
            Commands::List => commands::list(&db),
        };

        // Output Error.
        if let Err(err) = result {
            eprintln!("\x1b[31mError:\x1b[39m {}", err);
        }
    }

}
