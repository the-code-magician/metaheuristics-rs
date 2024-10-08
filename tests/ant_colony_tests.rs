use metaheuristics::ant_colony::AntColony;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::BasicArchive;
use metaheuristics::individuals::TourIndividual;

#[test]
fn test_ant_colony_optimization() {
    let distances = vec![
        vec![0.0, 2.0, 2.0, 1.0],
        vec![2.0, 0.0, 4.0, 3.0],
        vec![2.0, 4.0, 0.0, 5.0],
        vec![1.0, 3.0, 5.0, 0.0],
    ];

    let aco = AntColony::new(10, 100, 1.0, 5.0, 0.5, distances.clone());
    let mut archive = BasicArchive::new(3);

    aco.optimize::<BasicArchive<TourIndividual>>(&mut archive);

    assert!(archive.get_best().is_some());
    if let Some(best_tour) = archive.get_best() {
        let fitness = best_tour.fitness();
        assert!(fitness >= 0.0);
    }
}
