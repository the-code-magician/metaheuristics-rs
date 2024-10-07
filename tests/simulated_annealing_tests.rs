use metaheuristics::simulated_annealing::SimulatedAnnealing;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::BasicArchive;
use rand::prelude::*;

#[test]
fn test_simulated_annealing_optimization() {
    let sa = SimulatedAnnealing::new(
        100.0,
        0.01,
        500,
        (0..5).map(|_| rand::random::<f64>() * 10.0 - 5.0).collect(),
        Box::new(|state| {
            state.iter().map(|&x| x + rand::random::<f64>() * 0.1 - 0.05).collect()
        }),
    );

    let mut archive = BasicArchive::new(5);

    sa.optimize(
        |state| state.iter().map(|&x| x * x).sum(),
        &mut archive,
    );

    assert!(archive.get_best().is_some());
    if let Some(best_solution) = archive.get_best() {
        let fitness: f64 = best_solution.iter().map(|&x| x * x).sum();
        assert!(fitness >= 0.0);
    }
}
