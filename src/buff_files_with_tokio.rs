use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn file_work() -> Result<(), io::Error> {
    let mut file = File::open("dev.txt").await?;

    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;

    println!("len = {}", contents.len());

    let mut file2 = File::create("new-dev-2.txt").await?;
    file2.write_all(contents.as_ref()).await?;

    Ok(())
}