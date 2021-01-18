mod arrays;
mod jsons;
mod files;
mod iterators;

fn main() {
    // arrays::test1();
    // arrays::test2();
    // arrays::test3();
    // arrays::test4();
    // arrays::test5();
    //arrays::test6();
    let t: Vec<_> = iterators::people().take(10).collect();
    println!("{:?}", t)
}
