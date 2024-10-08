// src/individuals.rs

use rand::prelude::*;
use crate::individual::{Individual, FitnessValue, Crossover, Mutate, Neighbor};
use std::f64;

#[derive(Clone)]
pub struct NumericIndividual {
    pub genes: Vec<f64>,
}

// Implementations for NumericIndividual...

impl Individual for NumericIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        self.genes.iter().map(|&x| x * x).sum()
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
        let mut rng = thread_rng();
        let genes = (0..5).map(|_| rng.gen_range(-5.0..5.0)).collect();
        Self { genes }
    }
}

#[derive(Clone)]
pub struct BitStringIndividual {
    pub bits: Vec<bool>,
}

// Implementations for BitStringIndividual...

impl Individual for BitStringIndividual {
    type Fitness = usize;

    fn fitness(&self) -> Self::Fitness {
        self.bits.iter().filter(|&&bit| bit).count()
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
        let mut rng = thread_rng();
        let bits = (0..20).map(|_| rng.gen::<bool>()).collect();
        Self { bits }
    }
}
