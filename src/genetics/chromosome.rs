// chromosome.rs
//
// Implements a chromosome for a genetic algorithms implementation

use csv::Reader;
use rand::Rng;

use std::collections::HashMap;
use std::fs::read_dir;
use std::io::Error;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Chromosome {
    pub genes: Vec<f32>,
    pub fitness: f32,
}

impl Chromosome {
    pub fn new() -> Self {
        let fitness = 0.0;

        let mut genes: Vec<f32> = Vec::new();
        genes.fill(rand::thread_rng().gen());

        return Self { genes, fitness };
    }

    fn rmse(actual: &Vec<f32>, predictor: &Vec<f32>) -> f32 {
        (actual
            .iter()
            .zip(predictor)
            .map(|(a, p)| (p - a).powi(2))
            .sum::<f32>()
            / actual.len() as f32)
            .sqrt()
    }

    pub fn calculate_sample_fitness(&self, data: &Vec<f32>) -> f32 {
        assert!(
            data.len() > self.genes.len(),
            "There is not enough data in the sample to use the selected number of genes"
        );

        let mut predictor_dataset: Vec<f32> = Vec::new();
        for i in 0..(data.len() - self.genes.len()) {
            let data_slice = &data[i..i + self.genes.len()];
            let prediction_sum: f32 = data_slice
                .iter()
                .zip(self.genes.clone())
                .map(|(x, y)| x * y)
                .sum();
            predictor_dataset.push(prediction_sum / self.genes.len() as f32);
        }

        assert!(
            predictor_dataset.len() == data.len() - self.genes.len(),
            "The predictor dataset was not the right length (data.len() - self.genes.len())"
        );

        Chromosome::rmse(&data[self.genes.len()..].to_vec(), &predictor_dataset)
    }

    pub fn calculate_dataset_fitness(&mut self, data_dir: &Path) -> Result<f32, Error> {
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

        let mut fitness_sum: f32 = 0.0;
        for (_, d) in data.iter() {
            fitness_sum += Chromosome::calculate_sample_fitness(self, d);
        }
        Ok(fitness_sum / data.len() as f32)
    }
}
