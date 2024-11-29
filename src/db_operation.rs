use std::{fs, io};
use dirs::home_dir;

pub const TODO_DB_FILENAME: &str = ".tododb";

// Get the db_file path with user_home_path
pub fn get_db_file_path() -> std::path::PathBuf {
    home_dir()
        .map(|it| it.join(TODO_DB_FILENAME))
        .expect("Failed to find the home directory")
}

// Check if db_file is exists
pub fn db_file_exists() -> bool {
    let path = get_db_file_path();
    fs::metadata(path)
        .map(|meta| meta.is_file())
        .unwrap_or(false)
}

// Create db_file
pub fn create_db_file() -> io::Result<()> {
    let path = get_db_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(path)?;
    Ok(())
}

// Check if the database file exists, otherwise create it
pub fn check_db_file_or_create() -> io::Result<()> {
    if !db_file_exists() {
        if let Err(e) = create_db_file() {
            eprintln!("Failed to create database fiel: {}",e);
            return Err(e);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_db_file_path() {
        let path = get_db_file_path();
        // Output to view
        // println!("{:?}",path);
        assert!(path.ends_with(TODO_DB_FILENAME), "Path should end with the db filename");
    }

    #[test]
    fn test_db_exists() {
        // Ensure file exists
        check_db_file_or_create().unwrap();
        assert!(db_file_exists(), "File should exist after calling check_db_file");
    }

    #[test]
    fn test_create_db_file() {
        // Remove file to ready test
        let path = get_db_file_path();
        if path.exists() {
            fs::remove_file(&path).unwrap();
        }

        assert!(!db_file_exists(), "File should not exist initially");
        check_db_file_or_create().unwrap();
        assert!(db_file_exists(), "File should exist after creation");
    }

    // Fn check_db_file_or_create has tested in last several tests.
}
