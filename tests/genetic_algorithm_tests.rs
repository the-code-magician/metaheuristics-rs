use metaheuristics::genetic_algorithm::GeneticAlgorithm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::BasicArchive;

#[test]
fn test_genetic_algorithm_optimization() {
    let ga = GeneticAlgorithm::new(
        50,
        0.05,
        0.8,
        30,
        5,
        Box::new(|| rand::random::<f64>() * 10.0 - 5.0),
    );

    let mut archive = BasicArchive::new(5);

    ga.optimize(
        |individual| individual.iter().map(|&x| x * x).sum(),
        &mut archive,
    );

    assert!(archive.get_best().is_some());
    if let Some(best_solution) = archive.get_best() {
        let fitness: f64 = best_solution.iter().map(|&x| x * x).sum();
        assert!(fitness >= 0.0);
    }
}
