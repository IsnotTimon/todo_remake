pub struct Record {
    pub id: i32,
    pub content: String,
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let fields: Vec<&str> = value.split(",").collect();
        // Handle empty cases.
        if fields.is_empty() || fields[0].trim().is_empty() {
            return Record {
                id: 0,
                content: "".to_string(),
            };
        }
        // Handle possible errors, add default value
        let id = fields[0].trim().parse::<i32>().unwrap_or(0);
        let content = if fields.len() > 1 {
            fields[1..].join(",").trim().to_string()
        } else {
            "".to_string()
        };
        Record { id, content }
    }
}


use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek, Write};

use crate::db_operation::{check_db_file_or_create, get_db_file_path};
pub struct Database {
    pub file: File,
}

impl Database {
    // Create & open Database File
    pub fn open_database_file() -> io::Result<Database> {
        check_db_file_or_create()?;

        let db_file_path = get_db_file_path();
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(db_file_path)?;
        Ok(Database { file })
    }

    // Add record to Database File
    pub fn add_record(& mut self, record: &Record) -> io::Result<()> {
        // Ensure data write to file_end
        self.file.seek(io::SeekFrom::End(0))?;
        writeln!(self.file,"{} {}",record.id,record.content)?;
        // Write data to disk
        self.file.flush()
    }

    // Read records & Parse to Vec<Record>
    pub fn read_records(&self) -> io::Result<Vec<Record>> {
        let reader = BufReader::new(&self.file);
        reader
            .lines()
            .map(|packed_line| packed_line.map(|line| Record::from(line.as_str())))
            .collect()
    }

    // Delete record
    pub fn remove_record(&mut self, id: i32) -> Result<(), std::io::Error> {
        let reader = BufReader::new(&self.file);

        let mut lines = reader.lines().enumerate();
        let target_line = lines.find_map(|(i, line)| {
            line.ok().and_then(|content| {
                let record = Record::from(content.as_str());
                if record.id == id {
                    Some(i)
                } else {
                    None
                }
            })
        });
        let target_index = match target_line {
            Some(index) => index,
            None => {
                return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("No such record:{}",id)));
            }
        };
        let db_file = get_db_file_path();
        let contents = fs::read_to_string(&db_file)?;

        let new_contents = contents
            .lines()
            .enumerate()
            .filter(|(i, _)| *i != target_index)
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n");

        self.file.seek(std::io::SeekFrom::Start(0))?;
        self.file.write_all(new_contents.as_bytes())?;
        self.file.set_len(new_contents.len() as u64)?;

        Ok(())
    }

}



#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_database_operations() {
        // let mut db = Database::open_database_file().unwrap();

        // 添加记录

        // 读取记录

        // 删除记录

    }
}
