use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;
//
// static LOREM_IPSUM: &str =
//     "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
// tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
// quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
// consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
// cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
// proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
// ";
//
//
// pub fn test1() {
//     let path = Path::new("lorem_ipsum.txt");
//     let display = path.display();
//
//     // Open a file in write-only mode, returns `io::Result<File>`
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}: {}", display, why),
//         Ok(file) => file,
//     };
//
//     // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
//     match file.write_all(LOREM_IPSUM.as_bytes()) {
//         Err(why) => panic!("couldn't write to {}: {}", display, why),
//         Ok(_) => println!("successfully wrote to {}", display),
//     }
//
//     // // Read the file contents into a string, returns `io::Result<usize>`
//     // let mut s = String::new();
//     // match file.read_to_string(&mut s) {
//     //     Err(why) => panic!("couldn't read {}: {}", display, why),
//     //     Ok(_) => print!("{} contains:\n{}", display, s),
//     // }
// }
//
// pub fn test2() {
//     let path = Path::new("lorem_ipsum.txt");
//     let display = path.display();
//
//     // Open the path in read-only mode, returns `io::Result<File>`
//     let mut file = match File::open(&path) {
//         Err(why) => panic!("couldn't open {}: {}", display, why),
//         Ok(file) => file,
//     };
//
//     // Read the file contents into a string, returns `io::Result<usize>`
//     let mut s = String::new();
//     match file.read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read {}: {}", display, why),
//         Ok(_) => print!("{} contains:\n{}", display, s),
//     }
// }

// pub fn test3() {
//     writeFileDirty("test.txt".as_ref(), LOREM_IPSUM.as_bytes());
// }
//
// pub fn writeFileDirty(name: &str, bytes: &[u8]) {
//     let path = Path::new(name);
//     let display = path.display();
//
//     // Open a file in write-only mode, returns `io::Result<File>`
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}: {}", display, why),
//         Ok(file) => file,
//     };
//
//     file.write_all(bytes);
// }