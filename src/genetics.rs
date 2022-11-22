// genetics.rs
//
// Provides support for training and testing of a genetic algorithms framework

mod chromosome;
use chromosome::Chromosome;

pub struct Genetics {
    population: Vec<Chromosome>,
}

impl Genetics {
    pub fn new(size: i8) -> Self {
        let mut population: Vec<Chromosome> = Vec::new();
        for _ in 0..size {
            population.push(Chromosome::new());
        }
        Self { population }
    }
    pub fn get_chromosomes(&self) -> Vec<Chromosome> {
        self.population.clone()
    }
}
