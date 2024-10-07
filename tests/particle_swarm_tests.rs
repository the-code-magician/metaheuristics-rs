use metaheuristics::particle_swarm::ParticleSwarm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::BasicArchive;

#[test]
fn test_particle_swarm_optimization() {
    let pso = ParticleSwarm::new(
        20,
        3,
        50,
        0.5,
        1.5,
        1.5,
    );

    let mut archive = BasicArchive::new(5);

    pso.optimize(
        |position| position.iter().map(|&x| x * x).sum(),
        &mut archive,
    );

    assert!(archive.get_best().is_some());
    if let Some(best_solution) = archive.get_best() {
        let fitness: f64 = best_solution.iter().map(|&x| x * x).sum();
        assert!(fitness >= 0.0);
    }
}
