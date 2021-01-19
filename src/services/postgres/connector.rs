use crate::services::db::Database;

pub struct Postgres {
    url: String
}

impl Database<bool, String> for Postgres {
    fn connect(&mut self) -> Result<bool, String> {
        so(self.url.as_str());
        Ok(true)
    }

    fn disconnect(&mut self) -> Result<bool, String> {
        Ok(true)
    }
}

impl Postgres {
    pub fn new() -> Postgres {
        Postgres { url: String::from("postgres://") }
    }
}

pub fn so(data: &str) {
    println!("{}", data);
}