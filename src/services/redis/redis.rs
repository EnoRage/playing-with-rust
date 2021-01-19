use crate::services::db::Database;

pub struct Redis {
    url: String
}

impl Database<bool, String> for Redis {
    fn connect(&self) -> Result<bool, String> {
        so(self.url.as_str());
        Ok(true)
    }

    fn disconnect(&self) -> Result<bool, String> {
        Ok(true)
    }
}

impl Redis {
    pub fn new() -> Redis {
        Redis { url: String::from("redis://") }
    }
}

pub fn so(data: &str) {
    println!("{} redis", data);
}