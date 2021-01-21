// // use std::error::Error;
//
// // mod arrays;
// pub mod jsons;
// // mod files;
// // mod iterators;
// // mod buff_files;
// // mod buff_files_with_tokio;
//
// mod services;
//
// // #[tokio::main]
// // async fn main() -> Result<(), Box<dyn Error>> {
// //     // arrays::test1();
// //     // arrays::test2();
// //     // arrays::test3();
// //     // arrays::test4();
// //     // arrays::test5();
// //     // arrays::test6();
// //
// //     // jsons::test1();
// //
// //     // files::test1();
// //     // files::test2();
// //     //files::test3();
// //
// //     // let people_vec: Vec<_> = iterators::people().take(1000).collect();
// //     // //println!("{:?}", t)
// //     // let json_peoples = serde_json::to_string(&people_vec).unwrap().into_bytes();
// //     //
// //     // files::writeFileDirty("dev.txt".as_ref(), json_peoples.as_ref());
// //     //
// //     // buff_files::test3();
// //
// //     let result = buff_files_with_tokio::file_work().await;
// //
// //     Ok(())
// // }
// //
//
// fn main() {
//     services::db::init_and_connect();
// }

use futures::future::try_join_all;
use reqwest::Client;
use serde_json::Value;
use std::{error::Error, time::Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().build()?;
    // Start timing.
    let now = Instant::now();
    // Get values.
    let mut gets = Vec::new();
    // Default to few requests to be nice to jsonplaceholder.
    for id in 1..=3 {
        let get = get_todo(&client, id);
        gets.push(get);
    }
    let results = try_join_all(gets).await?;
    // Report time and result.
    println!("Elapsed: {} seconds", now.elapsed().as_secs_f64());
    println!("Result: {:#?}", results.last().unwrap());
    Ok(())
}

async fn get_todo(client: &Client, id: i32) -> Result<Value, Box<dyn Error>> {
    let base = "https://jsonplaceholder.typicode.com/todos";
    let address = format!("{}/{}", base, id);
    let result = client.get(&address).send().await?.json().await?;
    Ok(result)
}