use metaheuristics::ant_colony::AntColony;
use metaheuristics::Optimizer;
use metaheuristics::observer::Observer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::Individual;
use metaheuristics::individuals::TourIndividual;
use metaheuristics::logging_observers::LoggingObserver;
use metaheuristics::distribution_observers::DistributionObserver;

#[test]
fn test_ant_colony_optimization() {
    let distances = vec![
        vec![0.0, 2.0, 2.0, 1.0],
        vec![2.0, 0.0, 4.0, 3.0],
        vec![2.0, 4.0, 0.0, 5.0],
        vec![1.0, 3.0, 5.0, 0.0],
    ];

    let aco = AntColony::new(10, 100, 1.0, 5.0, 0.5, distances.clone());
    let mut archive: BasicArchive<TourIndividual> = BasicArchive::new(3);

    let mut observers: Vec<dyn Observer<TourIndividual>> = [
        LoggingObserver<TourIndividual>::new(true, true, true),
        DistributionObserver<TourIndividual>::new(),
    ];
    
    aco.optimize(&mut archive, &mut observers);

    assert!(archive.get_best().is_some());

    if let Some(best_tour) = archive.get_best() {
        let fitness = best_tour.fitness();
        assert!(fitness >= 0.0);
    }
}
