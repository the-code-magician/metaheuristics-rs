// src/individuals.rs

use rand::prelude::*;
use crate::individual::{Individual, FitnessValue, Crossover, Mutate, Neighbor};
use std::f64;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct BitStringIndividual {
    pub bits: Vec<bool>,
}

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

#[derive(Clone)]
struct ParticleIndividual {
    position: Vec<f64>,
    velocity: Vec<f64>,
}

impl ParticleIndividual {
    fn new(dimensions: usize) -> Self {
        let mut rng = thread_rng();
        let position = (0..dimensions).map(|_| rng.gen_range(-5.0..5.0)).collect();
        let velocity = (0..dimensions).map(|_| rng.gen_range(-1.0..1.0)).collect();
        Self { position, velocity }
    }
}

impl Individual for ParticleIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        self.position.iter().map(|&x| x * x).sum()
    }
}

#[derive(Clone)]
struct TourIndividual {
    tour: Vec<usize>,
    distances: Vec<Vec<f64>>,
}

impl TourIndividual {
    fn new(num_nodes: usize, distances: Vec<Vec<f64>>) -> Self {
        let mut tour = (0..num_nodes).collect::<Vec<_>>();
        let mut rng = thread_rng();
        tour.shuffle(&mut rng);
        Self { tour, distances }
    }
}

impl Individual for TourIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        let mut total_distance = 0.0;
        for i in 0..self.tour.len() - 1 {
            let from = self.tour[i];
            let to = self.tour[i + 1];
            total_distance += self.distances[from][to];
        }
        // Return to start
        total_distance += self.distances[self.tour[self.tour.len() - 1]][self.tour[0]];
        total_distance
    }
}