use metaheuristics::genetic_algorithm::GeneticAlgorithm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::BasicArchive;
use metaheuristics::individuals::BitStringIndividual;

#[test]
fn test_genetic_algorithm_bitstring_individual() {
    let ga = GeneticAlgorithm::new(50, 0.05, 0.8, 30);
    let mut archive = BasicArchive::new(5);
    let mut observers = [];

    ga.optimize(&mut archive, &mut observers);

    assert!(archive.get_best().is_some());
    if let Some(best_individual) = archive.get_best() {
        let fitness = best_individual.fitness();
        assert!(fitness <= 20);
    }
}
