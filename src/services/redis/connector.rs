use crate::services::db::{DatabaseConnector, DatabaseCR};
use redis::{Commands, RedisError, FromRedisValue, RedisResult};

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

impl DatabaseCR<String, String, RedisError> for Redis {
    fn get(&mut self, key: String) -> Result<String, RedisError> {
        let mut conn = self.connection.as_mut().unwrap();
        let res: String = conn.get(key)?;
        Ok(res)
    }

    fn set(&mut self, key: String, value: String) -> Option<RedisError> {
        let mut conn = self.connection.as_mut().unwrap();
        let res: RedisResult<()> = conn.set(key, value);
        None
    }
}

impl Redis {
    pub fn new() -> Redis {
        Redis { url: String::from("redis://127.0.0.1:6379"), connection: None }
    }
}