mod arrays;
mod iterators;

fn main() {
    let t: Vec<_> = iterators::people().take(10).collect();
    println!("{:?}", t)
}
