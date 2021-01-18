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
use std::thread::sleep;
use std::time::Duration;
use std::io::Error;

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

    let mut nf = File::create("new-dev.txt").unwrap();

    let f_len = f.metadata().unwrap().len();
    let mut buffer = [0; 1000];

    for x in (0..f_len).step_by(1000) {
        let diff = f_len - x;
        if (diff) < 1000 {
            // let mut f = vec![0 as u8; diff as usize];
            // let rf =&mut f[.. diff as usize];
            f.read_exact(&mut buffer[..diff as usize]);

            match nf.write(&mut buffer[..diff as usize]) {
                Ok(_) => {println!("The bytes: {:?} were written", &mut buffer[..diff as usize])}
                Err(_) => {}
            };
        }
        else  {
            f.read_exact(&mut buffer[..]);

            match nf.write(&buffer) {
                Ok(_) => {println!("The bytes: {:?} were written", &buffer[..])}
                Err(_) => {}
            };
        }


        // sleep(Duration::from_secs(1));
        // println!("The bytes: {:?}", &buffer[..]);
    }
    println!("finished")
}