mod genetics;

use genetics::Genetics;

use csv::Reader;
use rand::Rng;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;
use std::time::Instant;

static FITNESS_VALUE: f32 = 4.05;
static SELECTION_RATES: (i8, i8, i8) = (50, 25, 25);
//static POPULATION_SIZE: i8 = 60;
//static CROSSOVER_RATE: i8 = 10;
//static MUTATION_RATE: (i8, i8) = (10, 5);

fn read_in_data(path: &Path) -> HashMap<std::path::PathBuf, Vec<f32>> {
    let mut data: HashMap<std::path::PathBuf, Vec<f32>> = HashMap::new();
    for f in read_dir(path).unwrap() {
        let data_file = f.unwrap().path();
        if data_file.exists() && !data_file.is_dir() {
            let mut rdr = Reader::from_path(&data_file).unwrap();
            let mut temp_data: Vec<f32> = Vec::new();

            while let Some(result) = rdr.records().next() {
                let record = result.unwrap();
                temp_data.push(record[1].parse().unwrap());
            }

            data.insert(data_file, temp_data);
        }
    }
    data
}

fn main() {
    // TRAINING
    let POPULATION_SIZE: i8 = rand::thread_rng().gen_range(10..60);
    let CROSSOVER_RATE: i8 = rand::thread_rng().gen_range(5..25);
    let MUTATION_RATE: (i8, i8) = (
        rand::thread_rng().gen_range(1..15),
        rand::thread_rng().gen_range(1..10),
    );

    let mut g = Genetics::new(POPULATION_SIZE);
    let mut min_element_index: usize;

    let training_data_path = Path::new("training_data");
    assert!(training_data_path.exists());

    let data = read_in_data(training_data_path);

    let start = Instant::now();

    let mut best_chromosome_fitnesses: Vec<f32> = Vec::new();
    loop {
        g.calculate_dataset_fitness(&data);

        let result: Option<usize> = g
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| (a.fitness).total_cmp(&b.fitness))
            .map(|(index, _)| index);

        match result {
            Some(x) => min_element_index = x,
            None => panic!("No minimum fitness in the population! {:?}", g),
        }

        best_chromosome_fitnesses.push(g.population[min_element_index].fitness);

        if g.population[min_element_index].fitness < FITNESS_VALUE
            || start.elapsed().as_secs() > 3600
        {
            break;
        }

        g.selection(&SELECTION_RATES);
        g.crossover(&CROSSOVER_RATE);
        g.mutation(&MUTATION_RATE);
    }

    // TESTING

    let best_chromosome = &mut g.population[min_element_index];

    println!(
        "Best Chromosome Fitnesses:\n{:?}",
        best_chromosome_fitnesses
    );
    println!("Best chromosome weights: {:?}", best_chromosome);

    let good_testing_data_path = Path::new("good_testing_data");
    let bad_testing_data_path = Path::new("bad_testing_data");

    let good_testing_data = read_in_data(good_testing_data_path);
    let bad_testing_data = read_in_data(bad_testing_data_path);

    let fitness_values = (
        best_chromosome.fitness,
        best_chromosome
            .calculate_dataset_fitness(&good_testing_data)
            .unwrap(),
        best_chromosome
            .calculate_dataset_fitness(&bad_testing_data)
            .unwrap(),
    );

    println!(
        "Final fitness values: Training {}, Testing (good) {}, Testing (bad) {}",
        fitness_values.0, fitness_values.1, fitness_values.2
    );
}
