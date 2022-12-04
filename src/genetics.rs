// genetics.rs
//
// Provides support for training and testing of a genetic algorithms framework

mod chromosome;
use chromosome::Chromosome;
use csv::Reader;

use std::collections::HashMap;
use std::fs::read_dir;
use std::io::Error;
use std::path::Path;

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

    pub fn calculate_dataset_fitness(&mut self, data_dir: &Path) -> Result<(), Error> {
        let mut data_files: Vec<std::path::PathBuf> = Vec::new();
        for f in read_dir(data_dir)? {
            let path = f?.path();
            if path.exists() && !path.is_dir() {
                data_files.push(path)
            }
        }

        let mut data: HashMap<&std::path::PathBuf, Vec<f32>> = HashMap::new();
        for df in data_files.iter() {
            let mut rdr = Reader::from_path(df)?;
            let mut temp_data: Vec<f32> = Vec::new();

            while let Some(result) = rdr.records().next() {
                let record = result?;
                temp_data.push(record[1].parse().unwrap());
            }

            data.insert(df, temp_data.clone());
        }

        for c in self.population.iter_mut() {
            let mut fitness_sum: f32 = 0.0;
            for (_, d) in data.iter() {
                fitness_sum += c.calculate_sample_fitness(d);
            }
            c.fitness = fitness_sum / data.len() as f32;
        }

        Ok(())
    }
}
