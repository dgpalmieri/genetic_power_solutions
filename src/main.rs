mod genetics;

use genetics::Genetics;

use std::path::Path;

static POPULATION_SIZE: i8 = 60;
static FITNESS_VALUE: f32 = 0.0;
static SELECTION_RATES: (i8, i8, i8) = (0, 0, 0);
static CROSSOVER_RATE: i8 = 0;
static MUTATION_RATE: (i8, i8) = (0, 0);

fn main() {
    // TRAINING

    let mut g = Genetics::new(POPULATION_SIZE);
    let mut min_element_index: usize;

    let data_path = Path::new("test");

    loop {
        match g.calculate_dataset_fitness(data_path) {
            Ok(x) => x,
            Err(x) => panic!("Could not set dataset fitness, {:?}", x),
        }

        let result: Option<usize> = g
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| (a.fitness).total_cmp(&b.fitness))
            .map(|(index, _)| index);

        match result {
            Some(x) => min_element_index = x,
            None => panic!("No minimum fitness in the population! {:?}", g),
        }

        if g.population[min_element_index].fitness < FITNESS_VALUE {
            break;
        }

        g.selection(SELECTION_RATES);
        g.crossover(CROSSOVER_RATE);
        g.mutation(MUTATION_RATE);
    }

    // TESTING

    let best_chromosome = &mut g.population[min_element_index];

    let good_testing_data = Path::new("test");
    let bad_testing_data = Path::new("test");
    let fitness_values = (
        best_chromosome.fitness,
        best_chromosome
            .calculate_dataset_fitness(good_testing_data)
            .unwrap(),
        best_chromosome
            .calculate_dataset_fitness(bad_testing_data)
            .unwrap(),
    );

    println!(
        "Final fitness values: Training {}, Testing (good) {}, Testing (bad) {}",
        fitness_values.0, fitness_values.1, fitness_values.2
    );
}
