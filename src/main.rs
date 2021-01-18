use std::error::Error;

// mod arrays;
// mod jsons;
mod files;
mod iterators;
mod buff_files;
mod buff_files_with_tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // arrays::test1();
    // arrays::test2();
    // arrays::test3();
    // arrays::test4();
    // arrays::test5();
    // arrays::test6();

    // jsons::test1();

    // files::test1();
    // files::test2();
    //files::test3();

    // let people_vec: Vec<_> = iterators::people().take(1000).collect();
    // //println!("{:?}", t)
    // let json_peoples = serde_json::to_string(&people_vec).unwrap().into_bytes();
    //
    // files::writeFileDirty("dev.txt".as_ref(), json_peoples.as_ref());
    //
    // buff_files::test3();

    let result = buff_files_with_tokio::file_work().await;

    Ok(())
}

