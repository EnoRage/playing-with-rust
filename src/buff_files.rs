// use std::io::Read;
// use std::fs::File;
//
// pub fn test1() {
//     let mut file = File::open("dev.txt").unwrap();
//     let length = 5;
//
//     let mut vec = Vec::with_capacity(length);
//     file.by_ref().take(length as u64).read_to_end(&mut vec).unwrap();
//
//     let mut the_rest = Vec::new();
//     file.read_to_end(&mut the_rest).unwrap();
// }

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs::Metadata;

pub fn test2() -> io::Result<()> {
    let mut f = File::open("dev.txt")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..])?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

pub fn test3() {
    let mut f = File::open("dev.txt").unwrap();

    let f_len = f.metadata().unwrap().len();

    let mut buffer = [0; 100];

    for x in (0..f_len).step_by(100) {
        f.read_exact(&mut buffer[..]);
        println!("The bytes: {:?}", &buffer[..]);
    }

}