use std::io;
use crate::{database::{Database, Record}, db_operation::get_db_file_path};

pub fn info() -> Result<(), io::Error> {
    let db_file = get_db_file_path();
    println!("This is a simple todo_list app.");
    println!("Your todo list is stored at: {}.",db_file.display());
    println!("Wish you success!");
    Ok(())
}

// Add a new todo item.
pub fn add(db: &mut Database, content: Option<String>) -> Result<(), io::Error> {
    let content = content.ok_or_else(||{
        io::Error::new(io::ErrorKind::InvalidInput, "Content is required for the todo item.")
    })?;
    println!("Adding a todo item: {}", content);

    let id = db
        .read_records()
        .unwrap()
        .last()
        .map_or(1, |record| record.id + 1);

    db.add_record(&Record { id, content: content.clone() })?;
    println!("📝 Item added!");

    Ok(())
}

// Remove a todo item by ID.
pub fn remove(db: &mut Database, id: Option<String>) -> Result<(), io::Error> {
    let id_str = id.ok_or_else(||{
        io::Error::new(io::ErrorKind::InvalidInput, "Need a id.")
    })?;
    let id = id_str.parse::<i32>().map_err(|_|{
        io::Error::new(io::ErrorKind::InvalidInput, "Need a good id.")
    })?;
    println!("Removing a todo item with ID: {}",id);
    db.remove_record(id)?;
    println!("🙅 Item removed successfully!\n");
    Ok(())
}

// List all todo items.
pub fn list(db: &Database) -> Result<(), io::Error> {
    let records = db.read_records()?;

    if records.is_empty() {
        println!("No records.");
        return Ok(());
    }
    for record in records {
        println!("🌟 {}: {}", record.id, record.content);
    }

    Ok(())
}