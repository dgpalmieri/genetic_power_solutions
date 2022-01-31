mod genetics;
use genetics::Genetics;

fn main() {
    let mut g : Genetics = Genetics::new("Hello, World!");
    g.hello();
}
