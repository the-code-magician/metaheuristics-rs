use metaheuristics::particle_swarm::ParticleSwarm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::observer::Observer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::Individual;
use metaheuristics::individuals::ParticleIndividual;
use metaheuristics::logging_observers::LoggingObserver;
use metaheuristics::distribution_observers::DistributionObserver;

#[test]
fn test_particle_swarm() {
    // Define the fitness function
    let fitness_function = |position: &Vec<f64>| -> f64 {
        // Sphere function: sum of squares
        position.iter().map(|&x| x * x).sum()
    };

    let pso = ParticleSwarm::new(
        30,             // swarm_size
        5,              // dimensions
        100,            // iterations
        0.5,            // inertia_weight
        1.5,            // cognitive_coeff
        1.5,            // social_coeff
        fitness_function,
    );

    let mut archive: BasicArchive<ParticleIndividual> = BasicArchive::new(5);

    let mut observers: Vec<dyn Observer<ParticleIndividual>> = Vec::from([
        LoggingObserver::new(true, true, true),
        DistributionObserver::new(),
    ]);

    pso.optimize(&mut archive, &mut observers);

    assert!(archive.get_best().is_some());

    if let Some(best_particle) = archive.get_best() {
        let fitness = best_particle.fitness();
        assert!(fitness >= 0.0);
    }
}
