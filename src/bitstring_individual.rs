use rand::prelude::*;
use crate::individual::{Individual, Crossover, Mutate, FitnessValue};

#[derive(Clone)]
pub struct BitStringIndividual {
    pub bits: Vec<bool>,
}

impl BitStringIndividual {
    pub fn new(length: usize) -> Self {
        let mut rng = thread_rng();
        let bits = (0..length).map(|_| rng.gen::<bool>()).collect();
        Self { bits }
    }
}

impl Individual for BitStringIndividual {
    type Fitness = usize;

    fn fitness(&self) -> Self::Fitness {
        // For example, maximize the number of ones
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
