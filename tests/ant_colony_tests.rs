use metaheuristics::ant_colony::AntColony;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::BasicArchive;

#[test]
fn test_ant_colony_optimization() {
    let distances = vec![
        vec![0.0, 2.0, 2.0, 1.0],
        vec![2.0, 0.0, 4.0, 3.0],
        vec![2.0, 4.0, 0.0, 5.0],
        vec![1.0, 3.0, 5.0, 0.0],
    ];

    let aco = AntColony::new(
        5,
        50,
        1.0,
        5.0,
        0.5,
        distances.clone(),
    );

    let mut archive = BasicArchive::new(3);

    aco.optimize(
        |tour| {
            let mut total_distance = 0.0;
            for i in 0..tour.len() - 1 {
                total_distance += distances[tour[i]][tour[i + 1]];
            }
            total_distance
        },
        &mut archive,
    );

    assert!(archive.get_best().is_some());
    if let Some(best_tour) = archive.get_best() {
        assert_eq!(best_tour.first(), best_tour.last());
    }
}
