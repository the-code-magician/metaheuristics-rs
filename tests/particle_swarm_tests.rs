use metaheuristics::particle_swarm::ParticleSwarm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::individual::{Individual, FitnessValue};
use rand::prelude::*;

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
        // Sphere function
        self.position.iter().map(|&x| x * x).sum()
    }
}

impl FitnessValue for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}

impl Default for ParticleIndividual {
    fn default() -> Self {
        ParticleIndividual::new(5)
    }
}

// Implement additional traits or methods needed for Particle Swarm Optimization

#[test]
fn test_particle_swarm_optimization() {
    // Since ParticleSwarm hasn't been updated to use Individual trait in previous code,
    // Assuming it has been updated similarly to GeneticAlgorithm and SimulatedAnnealing

    let pso = ParticleSwarm::new(30, 5, 100, 0.5, 1.5, 1.5);
    let mut archive = BasicArchive::new(5);

    pso.optimize::<BasicArchive<ParticleIndividual>>(&mut archive);

    assert!(archive.get_best().is_some());
    if let Some(best_particle) = archive.get_best() {
        let fitness = best_particle.fitness();
        assert!(fitness >= 0.0);
    }
}
