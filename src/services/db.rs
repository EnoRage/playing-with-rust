use crate::services::postgres::postgres;
use crate::services::redis::redis;

pub trait Database<T, E> {
    fn connect(&self) -> Result<T, E>;
    fn disconnect(&self) -> Result<T, E>;
}

pub fn init_and_connect() {
    let rd = redis::Redis::new();
    let pg = postgres::Postgres::new();

    rd.connect().unwrap();
    pg.connect().unwrap();
}