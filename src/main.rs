mod genetics;
use genetics::Genetics;

static FITNESS_VALUE: i8 = 0;
static SELECTION_RATES: (i8, i8, i8) = (0, 0, 0);
static CROSSOVER_RATE: i8 = 0;
static MUTATION_RATE: (i8, i8) = (0, 0);

fn main() {
    let g: Chromosome = Chromosome::new();
    println!("{:?}", g.get_genes());
}
