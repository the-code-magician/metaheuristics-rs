use metaheuristics::simulated_annealing::SimulatedAnnealing;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::observer::Observer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::Individual;
use metaheuristics::individual::Neighbor;
use metaheuristics::individuals::NumericIndividual;
use metaheuristics::logging_observers::LoggingObserver;
use metaheuristics::distribution_observers::DistributionObserver;
use rand::prelude::*;


#[test]
fn test_simulated_annealing_numeric_individual() {
    let sa = SimulatedAnnealing::new(100.0, 0.01, 500);
    let mut archive: BasicArchive<NumericIndividual> = BasicArchive::new(5);
    let mut observers = vec![
        LoggingObserver::new(true, true, true),
    ];

    sa.optimize(&mut archive, &mut observers);

    assert!(archive.get_best().is_some());
    if let Some(best_individual) = archive.get_best() {
        let fitness = best_individual.fitness();
        assert!(fitness >= 0.0);
    }
}
