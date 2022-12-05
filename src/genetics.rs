// genetics.rs
//
// Provides support for training and testing of a genetic algorithms framework

mod chromosome;
use chromosome::Chromosome;
use csv::Reader;
use rand::Rng;

use std::collections::HashMap;
use std::f64::INFINITY;
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
        let mut data: HashMap<std::path::PathBuf, Vec<f32>> = HashMap::new();
        for f in read_dir(data_dir)? {
            let data_file = f?.path();
            if data_file.exists() && !data_file.is_dir() {
                let mut rdr = Reader::from_path(&data_file)?;
                let mut temp_data: Vec<f32> = Vec::new();

                while let Some(result) = rdr.records().next() {
                    let record = result?;
                    temp_data.push(record[1].parse().unwrap());
                }

                data.insert(data_file, temp_data);
            }
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

    pub fn selection(&mut self, selection_rates: (i8, i8, i8)) -> () {
        assert!(selection_rates.0 + selection_rates.1 + selection_rates.2 == 100);
        let random_selection = selection_rates.0 / self.population.len() as i8;
        let tournament_selection = selection_rates.1 / self.population.len() as i8;
        let generate_new_selection = selection_rates.2 / self.population.len() as i8;

        let mut new_pop: Vec<Chromosome> = Vec::new();

        for _ in 0..random_selection {
            let i = rand::thread_rng().gen_range(0..self.population.len());
            new_pop.push(self.population.swap_remove(i));
        }

        let tourney_size = self.population.len() / 10;
        for _ in 0..tournament_selection {
            let initial_index =
                rand::thread_rng().gen_range(0..self.population.len() - tourney_size);
            let tourney_pop = self.population[initial_index..initial_index + tourney_size].to_vec();

            let mut winner_index = initial_index;
            let mut min_fitness = INFINITY;
            let mut slice_index = 0;
            for c in tourney_pop.iter() {
                if (c.fitness as f64) < min_fitness {
                    min_fitness = c.fitness as f64;
                    winner_index = initial_index + slice_index;
                }
                slice_index += 1;
            }

            new_pop.push(self.population.swap_remove(winner_index));
        }

        for _ in 0..generate_new_selection {
            let i = rand::thread_rng().gen_range(0..self.population.len());
            new_pop.insert(i, Chromosome::new());
        }
    }

    pub fn crossover(&self) -> () {}
    pub fn mutation(&self) -> () {}
}
