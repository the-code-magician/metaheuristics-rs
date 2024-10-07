use metaheuristics::genetic_algorithm::GeneticAlgorithm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::individual::{Individual, Crossover, Mutate, FitnessValue};
use rand::prelude::*;

#[derive(Clone)]
struct NumericIndividual {
    genes: Vec<f64>,
}

impl NumericIndividual {
    fn new(length: usize) -> Self {
        let mut rng = thread_rng();
        let genes = (0..length).map(|_| rng.gen_range(-5.0..5.0)).collect();
        Self { genes }
    }
}

impl Individual for NumericIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        // Sphere function: minimize sum of squares
        self.genes.iter().map(|&x| x * x).sum()
    }
}

impl FitnessValue for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}

impl Crossover for NumericIndividual {
    fn crossover(&self, other: &Self, rng: &mut ThreadRng) -> Self {
        let crossover_point = rng.gen_range(0..self.genes.len());
        let mut new_genes = self.genes[..crossover_point].to_vec();
        new_genes.extend_from_slice(&other.genes[crossover_point..]);
        Self { genes: new_genes }
    }
}

impl Mutate for NumericIndividual {
    fn mutate(&mut self, rng: &mut ThreadRng) {
        let mutation_point = rng.gen_range(0..self.genes.len());
        self.genes[mutation_point] += rng.gen_range(-0.1..0.1);
    }
}

impl Default for NumericIndividual {
    fn default() -> Self {
        NumericIndividual::new(5)
    }
}

#[test]
fn test_genetic_algorithm_numeric_individual() {
    let ga = GeneticAlgorithm::new(50, 0.05, 0.8, 30);
    let mut archive = BasicArchive::new(5);

    ga.optimize::<BasicArchive<NumericIndividual>>(&mut archive);

    assert!(archive.get_best().is_some());
    if let Some(best_individual) = archive.get_best() {
        let fitness = best_individual.fitness();
        assert!(fitness >= 0.0);
    }
}
