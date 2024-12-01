use std::io;
use crate::db_operation::get_db_file_path;

pub fn info() -> Result<(), io::Error> {
    let db_file = get_db_file_path();
    println!("This is a simple todo_list app.");
    println!("Your todo list is stored at: {}.",db_file.display());
    println!("Wish you success!");
    Ok(())
}

pub fn remove() -> Result<(), io::Error> {
    println!("😧!");
    Ok(())
}