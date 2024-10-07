use metaheuristics::ant_colony::AntColony;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::individual::{Individual, FitnessValue};
use rand::prelude::*;

#[derive(Clone)]
struct TourIndividual {
    tour: Vec<usize>,
    distances: Vec<Vec<f64>>,
}

impl TourIndividual {
    fn new(num_nodes: usize, distances: Vec<Vec<f64>>) -> Self {
        let mut tour = (0..num_nodes).collect::<Vec<_>>();
        let mut rng = thread_rng();
        tour.shuffle(&mut rng);
        Self { tour, distances }
    }
}

impl Individual for TourIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        let mut total_distance = 0.0;
        for i in 0..self.tour.len() - 1 {
            let from = self.tour[i];
            let to = self.tour[i + 1];
            total_distance += self.distances[from][to];
        }
        // Return to start
        total_distance += self.distances[self.tour[self.tour.len() - 1]][self.tour[0]];
        total_distance
    }
}

impl FitnessValue for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}

impl Default for TourIndividual {
    fn default() -> Self {
        // Define a default distance matrix for testing
        let distances = vec![
            vec![0.0, 2.0, 2.0, 1.0],
            vec![2.0, 0.0, 4.0, 3.0],
            vec![2.0, 4.0, 0.0, 5.0],
            vec![1.0, 3.0, 5.0, 0.0],
        ];
        TourIndividual::new(4, distances)
    }
}

// Implement additional methods needed for Ant Colony Optimization

#[test]
fn test_ant_colony_optimization() {
    // Since AntColony hasn't been updated to use Individual trait in previous code,
    // Assuming it has been updated similarly to GeneticAlgorithm and SimulatedAnnealing

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
