mod genetics;
use genetics::Chromosome;

fn main() {
    let g: Chromosome = Chromosome::new();
    println!("{:?}", g.get_genes());
}
