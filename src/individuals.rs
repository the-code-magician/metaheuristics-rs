use rand::prelude::*;
use crate::individual::{Individual, Crossover, Mutate, Neighbor};
use std::f64;

#[derive(Clone, Neighbor)]
pub struct NumericIndividual {
    pub genes: Vec<f64>,
}

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

impl Neighbor for NumericIndividual {
    fn neighbor(&mut self) -> Self {
        let mut rng = thread_rng();
        let genes = genes.map(|v| v + rng.gen_range(-1.0..1.0)).collect();
        Self { genes }
    }
}

#[derive(Clone)]
pub struct BitStringIndividual {
    pub bits: Vec<bool>,
}

impl Individual for BitStringIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        self.bits.iter().filter(|&&bit| bit).count() as f64
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


#[derive(Clone)]
pub struct ParticleIndividual {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub personal_best_position: Vec<f64>,
    pub personal_best_score: f64,
}

impl ParticleIndividual {
    pub fn new(dimensions: usize) -> Self {
        let mut rng = thread_rng();
        let position: Vec<f64> = (0..dimensions).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let velocity: Vec<f64> = (0..dimensions).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let personal_best_position = position.clone();
        let personal_best_score = f64::INFINITY;
        Self {
            position,
            velocity,
            personal_best_position,
            personal_best_score,
        }
    }

    /// Updates the fitness of the particle using the provided fitness function.
    /// If the new fitness is better than the personal best, it updates the personal best.
    pub fn update_fitness<F>(&mut self, fitness_function: &F)
    where
        F: Fn(&Vec<f64>) -> f64,
    {
        let fitness = fitness_function(&self.position);
        if fitness < self.personal_best_score {
            self.personal_best_score = fitness;
            self.personal_best_position = self.position.clone();
        }
    }
}

impl Individual for ParticleIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        self.personal_best_score
    }
}

impl Default for ParticleIndividual {
    fn default() -> Self {
        Self::new(2) // Default to 2 dimensions
    }
}

#[derive(Clone)]
pub struct TourIndividual {
    pub tour: Vec<usize>,
    pub length: f64,
}

impl TourIndividual {
    pub fn new(tour: Vec<usize>, length: f64) -> Self {
        Self { tour, length }
    }
}

impl Individual for TourIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        self.length
    }
}