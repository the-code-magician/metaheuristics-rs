use rand::prelude::*;
use crate::optimizer::Optimizer;
use crate::archive::Archive;
use crate::individual::{Individual, FitnessValue};
use std::f64;

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
        let position = (0..dimensions).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let velocity = (0..dimensions).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let personal_best_position = position.clone();
        let personal_best_score = f64::INFINITY;
        Self {
            position,
            velocity,
            personal_best_position,
            personal_best_score,
        }
    }

    pub fn update_fitness<F>(&mut self, fitness_function: F)
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

impl FitnessValue for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}

impl Default for ParticleIndividual {
    fn default() -> Self {
        Self::new(2) // Default to 2 dimensions
    }
}

pub struct ParticleSwarm<F>
where
    F: Fn(&Vec<f64>) -> f64,
{
    pub swarm_size: usize,
    pub dimensions: usize,
    pub iterations: usize,
    pub inertia_weight: f64,
    pub cognitive_coeff: f64,
    pub social_coeff: f64,
    pub fitness_function: F,
}

impl<F> ParticleSwarm<F>
where
    F: Fn(&Vec<f64>) -> f64,
{
    pub fn new(
        swarm_size: usize,
        dimensions: usize,
        iterations: usize,
        inertia_weight: f64,
        cognitive_coeff: f64,
        social_coeff: f64,
        fitness_function: F,
    ) -> Self {
        Self {
            swarm_size,
            dimensions,
            iterations,
            inertia_weight,
            cognitive_coeff,
            social_coeff,
            fitness_function,
        }
    }
}

impl Optimizer<ParticleIndividual> for ParticleSwarm<impl Fn(&Vec<f64>) -> f64 + Copy>
{
    fn optimize<A>(&self, archive: &mut A)
    where
        A: Archive<Solution = ParticleIndividual, Fitness = f64>,
    {
        let mut rng = thread_rng();

        let mut particles: Vec<ParticleIndividual> = (0..self.swarm_size)
            .map(|_| ParticleIndividual::new(self.dimensions))
            .collect();

        let mut global_best_position = vec![0.0; self.dimensions];
        let mut global_best_score = f64::INFINITY;

        for particle in &mut particles {
            particle.update_fitness(&self.fitness_function);
            let fitness = particle.fitness();

            if fitness < global_best_score {
                global_best_score = fitness;
                global_best_position = particle.position.clone();
            }

            archive.add(particle.clone());
        }

        for _ in 0..self.iterations {
            for particle in &mut particles {
                for d in 0..self.dimensions {
                    let rp = rng.gen::<f64>();
                    let rg = rng.gen::<f64>();

                    let cognitive_velocity = self.cognitive_coeff * rp * (particle.personal_best_position[d] - particle.position[d]);
                    let social_velocity = self.social_coeff * rg * (global_best_position[d] - particle.position[d]);

                    particle.velocity[d] = self.inertia_weight * particle.velocity[d]
                        + cognitive_velocity
                        + social_velocity;

                    particle.position[d] += particle.velocity[d];
                }

                particle.update_fitness(&self.fitness_function);
                let fitness = particle.fitness();

                if fitness < global_best_score {
                    global_best_score = fitness;
                    global_best_position = particle.position.clone();
                }

                archive.add(particle.clone());
            }
        }
    }
}
