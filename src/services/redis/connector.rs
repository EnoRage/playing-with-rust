use crate::services::db::{DatabaseConnector, DatabaseCR};
use redis::Commands;

pub struct Redis {
    url: String,
    connection: Option<redis::Connection>,
}

impl DatabaseConnector<(), redis::RedisError> for Redis {
    fn connect(&mut self) -> redis::RedisResult<()> {
        let client = redis::Client::open(self.url.as_str())?;
        let con = client.get_connection().unwrap();

        self.connection = Some(con);
        Ok(())
    }

    fn disconnect(&mut self) -> redis::RedisResult<()> {
        // some shit actually
        drop(self.connection.as_ref());
        Ok(())
    }
}

impl DatabaseCR<String, Vec<u8>, String> for Redis {
    fn get(&mut self, key: String) -> Result<Vec<u8>, String> {
        let result = *self
            .connection
            .as_ref()
            .unwrap()
            .get::<String, Vec<u8>>(key);
        Ok(result)
    }

    fn set(&mut self, key: String, value: Vec<u8>) -> Option<String> {
        *self.connection.unwrap().set::<String, Vec<u8>, ()>(key, value);
        None
    }
}

impl Redis {
    pub fn new() -> Redis {
        Redis { url: String::from("redis://127.0.0.1:6379"), connection: None }
    }
}