use metaheuristics::genetic_algorithm::GeneticAlgorithm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::observers::Observer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::individual::{Individual, Crossover, Mutate, FitnessValue};
use rand::prelude::*;

#[derive(Clone)]
struct CustomIndividual {
    data: Vec<char>,
}

impl CustomIndividual {
    fn new(target: &str) -> Self {
        let mut rng = thread_rng();
        let data = (0..target.len())
            .map(|_| rng.gen_range('a'..='z'))
            .collect();
        Self { data }
    }
}

impl Individual for CustomIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        let target = "hello";
        self.data
            .iter()
            .zip(target.chars())
            .filter(|&(a, b)| *a != b)
            .count() as f64
    }
}

impl Crossover for CustomIndividual {
    fn crossover(&self, other: &Self, rng: &mut ThreadRng) -> Self {
        let point = rng.gen_range(0..self.data.len());
        let mut new_data = self.data[..point].to_vec();
        new_data.extend_from_slice(&other.data[point..]);
        Self { data: new_data }
    }
}

impl Mutate for CustomIndividual {
    fn mutate(&mut self, rng: &mut ThreadRng) {
        let idx = rng.gen_range(0..self.data.len());
        self.data[idx] = rng.gen_range('a'..='z');
    }
}

impl Default for CustomIndividual {
    fn default() -> Self {
        CustomIndividual::new("hello")
    }
}

#[test]
fn test_genetic_algorithm_custom_individual() {
    let ga = GeneticAlgorithm::new(100, 0.1, 0.7, 50);
    let mut archive:BasicArchive<CustomIndividual>  = BasicArchive::new(5);

    let mut observers: Vec<Observer> = [];

    ga.optimize(&mut archive, &mut observers);

    assert!(archive.get_best().is_some());
}
