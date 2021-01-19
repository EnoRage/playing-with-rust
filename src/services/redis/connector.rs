use crate::services::db::Database;
use redis::ConnectionLike;

pub struct Redis {
    url: String,
    connection: Option<redis::Connection>
}

impl Database<(), redis::RedisError> for Redis {
    fn connect(&mut self) -> redis::RedisResult<()> {
        let client = redis::Client::open(self.url.as_str())?;
        let con = client.get_connection().unwrap();

        self.connection = Some(con);
        Ok(())
    }

    fn disconnect(&mut self) -> redis::RedisResult<()> {
        Ok(())
    }
}

impl Redis {
    pub fn new() -> Redis {
        Redis { url: String::from("redis://127.0.0.1:6379"), connection: None}
    }
}