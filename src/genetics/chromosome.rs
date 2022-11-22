// chromosome.rs
//
// Implements a chromosome for a genetic algorithms implementation

use rand::Rng;

pub struct Chromosome {
    genes: Vec<i32>,
}

impl Chromosome {
    pub fn new() -> Self {
        let mut genes: Vec<i32> = Vec::new();
        for _ in 0..60 {
            genes.push(rand::thread_rng().gen_range(0..10));
        }
        return Self { genes };
    }

    pub fn get_genes(&self) -> Vec<i32> {
        self.genes.clone()
    }

    pub fn calculate_sample_fitness(&self, file_path: &String) -> i8 {
        0
    }
}
