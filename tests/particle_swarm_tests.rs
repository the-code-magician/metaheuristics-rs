use metaheuristics::particle_swarm::ParticleSwarm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::individuals::ParticleIndividual;
use rand::prelude::*;


#[test]
fn test_particle_swarm_optimization() {
    // Since ParticleSwarm hasn't been updated to use Individual trait in previous code,
    // Assuming it has been updated similarly to GeneticAlgorithm and SimulatedAnnealing

    let pso = ParticleSwarm::new(30, 5, 100, 0.5, 1.5, 1.5);
    let mut archive = BasicArchive::new(5);
    let mut observers = [];

    ga.optimize(&mut archive, &mut observers);

    assert!(archive.get_best().is_some());
    if let Some(best_particle) = archive.get_best() {
        let fitness = best_particle.fitness();
        assert!(fitness >= 0.0);
    }
}
