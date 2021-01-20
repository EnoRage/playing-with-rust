use crate::services::db::{DatabaseConnector, DatabaseCR};
use redis::{Commands, RedisError, RedisResult};

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

impl DatabaseCR<String, Vec<u8>, RedisError> for Redis {
    fn get(&mut self, key: String) -> Result<Vec<u8>, RedisError> {
        Ok(self.connection.as_mut().unwrap().get(key).unwrap())
    }

    fn set(&mut self, key: String, value: Vec<u8>) -> Option<RedisError> {
        let conn = self.connection.as_mut().unwrap();
        let _res: RedisResult<()> = conn.set(key, value);
        None
    }
}

impl Redis {
    pub fn new() -> Redis {
        Redis { url: String::from("redis://127.0.0.1:6379"), connection: None }
    }
}