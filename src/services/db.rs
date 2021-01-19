use crate::services::postgres::connector as pg_connector;
use crate::services::redis::connector as redis_connector;

pub trait DatabaseConnector<T, E> {
    fn connect(&mut self) -> Result<T, E>;
    fn disconnect(&mut self) -> Result<T, E>;
}

pub trait DatabaseCR<TK, TV, E> {
    fn get(&mut self, key: TK) -> Result<TV, E>;
    fn set(&mut self, key: TK, value: TV) -> Option<E>;
}


pub fn init_and_connect() {
    let mut rd = redis_connector::Redis::new();
    let mut pg = pg_connector::Postgres::new();

    rd.connect().unwrap();
    pg.connect().unwrap();
}