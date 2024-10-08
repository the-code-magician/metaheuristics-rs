use rand::prelude::*;
use crate::optimizer::Optimizer;
use crate::archive::Archive;
use crate::individuals::ParticleIndividual;
use crate::observer::Observer;
use crate::Individual;
use std::f64;

pub struct ParticleSwarm<F>
where
    F: Fn(&Vec<f64>) -> f64 + Copy,
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
    F: Fn(&Vec<f64>) -> f64 + Copy,
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

impl<F> Optimizer<ParticleIndividual> for ParticleSwarm<F>
where
    F: Fn(&Vec<f64>) -> f64 + Copy,
{
    fn optimize<A, O>(&self, archive: &mut A, observers: &mut [O])
    where
        A: Archive<Solution = ParticleIndividual, Fitness = f64>,
        O: Observer<ParticleIndividual>,
    {
        let mut rng = thread_rng();

        let mut particles: Vec<ParticleIndividual> = (0..self.swarm_size)
            .map(|_| ParticleIndividual::new(self.dimensions))
            .collect();

        let mut global_best_position = vec![0.0; self.dimensions];
        let mut global_best_score = f64::INFINITY;

        for observer in observers.iter_mut() {
            observer.on_start();
            observer.on_iteration(0, &particles);
        }

        for particle in &mut particles {
            particle.update_fitness(&self.fitness_function);
            let fitness = particle.fitness();

            if fitness < global_best_score {
                global_best_score = fitness;
                global_best_position = particle.position.clone();
            }

            archive.add(particle.clone());
        }

        for iteration in 0..self.iterations {
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
            
            for observer in observers.iter_mut() {
                observer.on_iteration(iteration, &particles);
            }
        }

        for observer in observers.iter_mut() {
            observer.on_finish();
        }
    }
}
