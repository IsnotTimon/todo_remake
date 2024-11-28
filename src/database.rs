pub struct Record {
    pub id: i32,
    pub content: String,
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let fields: Vec<&str> = value.split(",").collect();
        if fields.is_empty() || fields[0].trim().is_empty() {
            return Record {
                id: 0,
                content: "".to_string(),
            };
        }
        let id = fields[0].trim().parse::<i32>().unwrap_or(0);
        let content = if fields.len() > 1 {
            fields[1..].join(",").trim().to_string()
        } else {
            "".to_string()
        };
        Record { id, content }
    }
}
