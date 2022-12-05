// genetics.rs
//
// Provides support for training and testing of a genetic algorithms framework

mod chromosome;
use chromosome::Chromosome;
use csv::Reader;
use rand::Rng;

use std::collections::HashMap;
use std::fs::read_dir;
use std::io::Error;
use std::path::Path;

#[derive(Debug)]
pub struct Genetics {
    pub population: Vec<Chromosome>,
}

impl Genetics {
    pub fn new(size: i8) -> Self {
        let mut population: Vec<Chromosome> = Vec::new();
        for _ in 0..size {
            population.push(Chromosome::new());
        }
        Self { population }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Chromosome> {
        self.population.iter()
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
        let mut random_selection = selection_rates.0 / self.population.len() as i8;
        let tournament_selection = selection_rates.1 / self.population.len() as i8;
        let generate_new_selection = selection_rates.2 / self.population.len() as i8;
        let mut selection_num = random_selection + tournament_selection + generate_new_selection;

        if selection_num != self.population.len() as i8 {
            if selection_num > self.population.len() as i8 {
                random_selection -= self.population.len() as i8 - selection_num;
            } else if selection_num < self.population.len() as i8 {
                random_selection += self.population.len() as i8 - selection_num;
            }
        }

        selection_num = random_selection + tournament_selection + generate_new_selection;
        assert!(selection_num == self.population.len() as i8);

        let mut new_pop: Vec<Chromosome> = Vec::with_capacity(self.population.capacity());

        for _ in 0..random_selection {
            let i = rand::thread_rng().gen_range(0..self.population.len());
            new_pop.push(self.population.swap_remove(i));
        }

        let mut tourney_size = self.population.len() / 10;
        if tourney_size == 0 {
            tourney_size = 2;
        }
        for _ in 0..tournament_selection {
            let initial_index =
                rand::thread_rng().gen_range(0..self.population.len() - tourney_size);
            let tourney_pop = self.population[initial_index..initial_index + tourney_size].to_vec();

            let mut winner_index = initial_index;
            let max_slice_index: Option<usize> = tourney_pop
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| (a.fitness).total_cmp(&b.fitness))
                .map(|(index, _)| index);

            match max_slice_index {
                Some(x) => winner_index += x,
                None => panic!(
                    "selection unable to find a maximum in slice! {:?}",
                    tourney_pop
                ),
            }

            new_pop.push(self.population.swap_remove(winner_index));
        }

        for _ in 0..generate_new_selection {
            let i = rand::thread_rng().gen_range(0..new_pop.len());
            new_pop.insert(i, Chromosome::new());
        }

        self.population = new_pop;
    }

    pub fn crossover(&mut self, crossover_rate: i8) -> () {
        for _ in 0..crossover_rate / self.population.len() as i8 {
            let crossover_a = rand::thread_rng().gen_range(0..self.population.len());
            let mut crossover_b = rand::thread_rng().gen_range(0..self.population.len());
            while crossover_a == crossover_b {
                crossover_b = rand::thread_rng().gen_range(0..self.population.len());
            }

            let slice_size =
                rand::thread_rng().gen_range(1..self.population[crossover_a].genes.len() / 2);
            let starting_index_a = rand::thread_rng()
                .gen_range(0..self.population[crossover_a].genes.len() - slice_size);
            let starting_index_b = rand::thread_rng()
                .gen_range(0..self.population[crossover_b].genes.len() - slice_size);

            let mut swap_slice = self.population[crossover_b].genes
                [starting_index_b..starting_index_b + slice_size]
                .to_vec();
            self.population[crossover_a].genes[starting_index_a..starting_index_a + slice_size]
                .swap_with_slice(&mut swap_slice);
            self.population[crossover_b].genes[starting_index_b..starting_index_b + slice_size]
                .swap_with_slice(&mut swap_slice);
        }
    }

    pub fn mutation(&mut self, mutation_rate: (i8, i8)) -> () {
        for _ in 0..mutation_rate.0 / self.population.len() as i8 {
            let m = rand::thread_rng().gen_range(0..self.population.len());
            for _ in 0..mutation_rate.1 / self.population[m].genes.len() as i8 {
                let new_weight_index =
                    rand::thread_rng().gen_range(0..self.population[m].genes.len());
                let new_weight: f32 = rand::thread_rng().gen();
                self.population[m].genes[new_weight_index] = new_weight;
            }
        }
    }
}
