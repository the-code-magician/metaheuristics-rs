use metaheuristics::genetic_algorithm::GeneticAlgorithm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::observer::Observer;
use metaheuristics::archive::{BasicArchive,Archive};
use metaheuristics::Individual;
use metaheuristics::individuals::BitStringIndividual;
use metaheuristics::logging_observers::LoggingObserver;
use metaheuristics::distribution_observers::DistributionObserver;

#[test]
fn test_genetic_algorithm_bitstring_individual() {
    let ga = GeneticAlgorithm::new(50, 0.05, 0.8, 30);
    let mut archive: BasicArchive<BitStringIndividual> = BasicArchive::new(5);
    
    let mut observers = vec![
        LoggingObserver::new(true, true, true),
        DistributionObserver::new(),
    ];

    ga.optimize(&mut archive, &mut observers);

    assert!(archive.get_best().is_some());
    if let Some(best_individual) = archive.get_best() {
        let fitness = best_individual.fitness();
        assert!(fitness <= 20.0);
    }
}
