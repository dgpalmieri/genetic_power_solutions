// chromosome.rs
//
// Implements a chromosome for a genetic algorithms implementation

use rand::Rng;

#[derive(Clone)]
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

    pub fn calculate_sample_fitness(&mut self, data: &Vec<f32>) -> () {
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

        self.fitness = Chromosome::rmse(&data[self.genes.len()..].to_vec(), &predictor_dataset);
    }
}
