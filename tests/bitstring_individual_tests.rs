use metaheuristics::genetic_algorithm::GeneticAlgorithm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::individual::{Individual, Crossover, Mutate, FitnessValue};
use rand::prelude::*;

#[derive(Clone)]
struct BitStringIndividual {
    bits: Vec<bool>,
}

impl BitStringIndividual {
    fn new(length: usize) -> Self {
        let mut rng = thread_rng();
        let bits = (0..length).map(|_| rng.gen::<bool>()).collect();
        Self { bits }
    }
}

impl Individual for BitStringIndividual {
    type Fitness = usize;

    fn fitness(&self) -> Self::Fitness {
        // Maximize the number of ones
        self.bits.iter().filter(|&&bit| bit).count()
    }
}

impl FitnessValue for usize {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl Crossover for BitStringIndividual {
    fn crossover(&self, other: &Self, rng: &mut ThreadRng) -> Self {
        let crossover_point = rng.gen_range(0..self.bits.len());
        let mut new_bits = self.bits[..crossover_point].to_vec();
        new_bits.extend_from_slice(&other.bits[crossover_point..]);
        Self { bits: new_bits }
    }
}

impl Mutate for BitStringIndividual {
    fn mutate(&mut self, rng: &mut ThreadRng) {
        let mutation_point = rng.gen_range(0..self.bits.len());
        self.bits[mutation_point] = !self.bits[mutation_point];
    }
}

impl Default for BitStringIndividual {
    fn default() -> Self {
        BitStringIndividual::new(20)
    }
}

#[test]
fn test_genetic_algorithm_bitstring_individual() {
    let ga = GeneticAlgorithm::new(50, 0.05, 0.8, 30);
    let mut archive = BasicArchive::new(5);

    ga.optimize::<BasicArchive<BitStringIndividual>>(&mut archive);

    assert!(archive.get_best().is_some());
    if let Some(best_individual) = archive.get_best() {
        let fitness = best_individual.fitness();
        assert!(fitness <= 20);
    }
}
