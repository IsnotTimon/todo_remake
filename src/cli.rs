use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about, propagate_version = true)]
pub struct Cli{
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // Show information about this app.
    Info,

    // Add a new todo item.
    Add {
        #[clap(help = "Content of the item to add.")]
        content: Option<String>,
    },

    // Remove a todo item by ID.
    Remove {
        #[clap(help = "ID of the item to remove.")]
        id: Option<String>,
    },

    // List all todo items.
    List,
}