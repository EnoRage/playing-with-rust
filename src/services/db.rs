use crate::services::postgres::connector as pg_connector;
use crate::services::redis::connector as redis_connector;

pub trait DatabaseConnector<T, E> {
    fn connect(&mut self) -> Result<T, E>;
    fn disconnect(&mut self) -> Result<T, E>;
}

pub trait DatabaseCR<TKey, TValue, TError> {
    fn get(&mut self, key: TKey) -> Result<TValue, TError>;
    fn set(&mut self, key: TKey, value: TValue) -> Option<TError>;
}


pub fn init_and_connect() {
    let mut rd = redis_connector::Redis::new();
    let mut pg = pg_connector::Postgres::new();

    rd.connect().unwrap();
    pg.connect().unwrap();
}