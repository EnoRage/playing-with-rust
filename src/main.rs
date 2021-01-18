mod arrays;
mod jsons;
mod files;

fn main() {
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

    let t: Vec<_> = iterators::people().take(10).collect();
    println!("{:?}", t);

    files::writeFileDirty("dev.txt".as_ref(), jsons::getJsonBinary().as_ref());
}

