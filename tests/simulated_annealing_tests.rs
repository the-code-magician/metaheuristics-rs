use metaheuristics::simulated_annealing::SimulatedAnnealing;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::individual::{Individual, Neighbor, FitnessValue};
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

impl Neighbor for NumericIndividual {
    fn neighbor(&self, rng: &mut ThreadRng) -> Self {
        let mut new_genes = self.genes.clone();
        let mutation_point = rng.gen_range(0..self.genes.len());
        new_genes[mutation_point] += rng.gen_range(-0.1..0.1);
        Self { genes: new_genes }
    }
}

impl Default for NumericIndividual {
    fn default() -> Self {
        NumericIndividual::new(5)
    }
}

#[test]
fn test_simulated_annealing_numeric_individual() {
    let sa = SimulatedAnnealing::new(100.0, 0.01, 500);
    let mut archive = BasicArchive::new(5);

    sa.optimize::<BasicArchive<NumericIndividual>>(&mut archive);

    assert!(archive.get_best().is_some());
    if let Some(best_individual) = archive.get_best() {
        let fitness = best_individual.fitness();
        assert!(fitness >= 0.0);
    }
}
