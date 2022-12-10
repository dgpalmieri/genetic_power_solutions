// chromosome.rs
//
// Implements a chromosome for a genetic algorithms implementation

use rand::Rng;

use std::collections::HashMap;
use std::io::Error;

#[derive(Clone, Debug)]
pub struct Chromosome {
    pub genes: Vec<f32>,
    pub fitness: f32,
}

impl Chromosome {
    pub fn new() -> Self {
        let fitness = 0.0;

        let mut genes: Vec<f32> = Vec::with_capacity(60);
        for _ in 0..genes.capacity() {
            genes.push(rand::thread_rng().gen());
        }

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
            predictor_dataset.push(prediction_sum / self.genes.iter().sum::<f32>());
        }

        assert!(
            predictor_dataset.len() == data.len() - self.genes.len(),
            "The predictor dataset was not the right length (data.len() - self.genes.len())"
        );

        Chromosome::rmse(&data[self.genes.len()..].to_vec(), &predictor_dataset)
    }

    pub fn calculate_dataset_fitness(
        &mut self,
        data: &HashMap<std::path::PathBuf, Vec<f32>>,
    ) -> f32 {
        let mut fitness_sum: f32 = 0.0;
        for (_, d) in data.iter() {
            fitness_sum += Chromosome::calculate_sample_fitness(self, d);
        }
        fitness_sum / data.len() as f32
    }
}

#[cfg(test)]
mod chromosome_tests {
    use super::*;

    #[test]
    fn test_rmse() {
        let one_a: Vec<f32> = vec![
            34.0, 37.0, 44.0, 47.0, 48.0, 48.0, 46.0, 43.0, 32.0, 27.0, 26.0, 24.0,
        ];
        let one_b: Vec<f32> = vec![
            37.0, 40.0, 46.0, 44.0, 46.0, 50.0, 45.0, 44.0, 34.0, 30.0, 22.0, 23.0,
        ];
        assert_eq!(Chromosome::rmse(&one_a, &one_b), 2.4324198);

        let two_a: Vec<f32> = vec![15.0, 18.0, 32.0, 1.0, 11.0];
        let two_b: Vec<f32> = vec![24.0, 16.0, 32.0, 55.0, 12.0];
        assert_eq!(Chromosome::rmse(&two_a, &two_b), 24.503061);

        let three_a: Vec<f32> = vec![1.0, 2.0, 3.0];
        let three_b: Vec<f32> = vec![1.0, 2.0, 3.0];
        assert_eq!(Chromosome::rmse(&three_a, &three_b), 0.0);
    }
}
