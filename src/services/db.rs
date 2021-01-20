use crate::services::postgres::connector as pg_connector;
use crate::services::redis::connector as redis_connector;

extern crate serde;
use serde::{Serialize, Deserialize};
use crate::jsons::Line;



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

    rd.set(String::from("test"), super::super::jsons::getJsonBinary());


    let v = rd.get(String::from("test")).unwrap();
    let vv: Vec<Line> = serde_json::from_slice(&v).unwrap();
    println!("{}", vv[0].desc)
}