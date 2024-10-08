use rand::prelude::*;
use crate::optimizer::Optimizer;
use crate::archive::Archive;
use crate::individual::{Individual, FitnessValue};

#[derive(Clone)]
pub struct TourIndividual {
    pub tour: Vec<usize>,
    pub length: f64,
}

impl TourIndividual {
    pub fn new(tour: Vec<usize>, length: f64) -> Self {
        Self { tour, length }
    }
}

impl Individual for TourIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        self.length
    }
}

pub struct AntColony {
    pub num_ants: usize,
    pub num_iterations: usize,
    pub alpha: f64,
    pub beta: f64,
    pub evaporation_rate: f64,
    pub distance_matrix: Vec<Vec<f64>>,
}

impl AntColony {
    pub fn new(
        num_ants: usize,
        num_iterations: usize,
        alpha: f64,
        beta: f64,
        evaporation_rate: f64,
        distance_matrix: Vec<Vec<f64>>,
    ) -> Self {
        Self {
            num_ants,
            num_iterations,
            alpha,
            beta,
            evaporation_rate,
            distance_matrix,
        }
    }
}

impl Optimizer<TourIndividual> for AntColony {
    fn optimize<A>(&self, archive: &mut A)
    where
        A: Archive<Solution = TourIndividual, Fitness = f64>,
    {
        let num_nodes = self.distance_matrix.len();
        let mut pheromones = vec![vec![1.0; num_nodes]; num_nodes];

        for _ in 0..self.num_iterations {
            let mut all_tours = Vec::new();
            let mut all_lengths = Vec::new();

            for _ in 0..self.num_ants {
                let (tour, length) = self.construct_solution(&pheromones);
                all_tours.push(tour.clone());
                all_lengths.push(length);

                let individual = TourIndividual::new(tour.clone(), length);
                archive.add(individual);
            }

            // Evaporate pheromones
            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    pheromones[i][j] *= 1.0 - self.evaporation_rate;
                    pheromones[i][j] = pheromones[i][j].max(0.1);
                }
            }

            // Update pheromones based on ant tours
            for (tour, length) in all_tours.iter().zip(all_lengths.iter()) {
                for k in 0..tour.len() - 1 {
                    let i = tour[k];
                    let j = tour[k + 1];
                    pheromones[i][j] += 1.0 / length;
                    pheromones[j][i] += 1.0 / length;
                }
            }
        }
    }
}

impl AntColony {
    fn construct_solution(&self, pheromones: &Vec<Vec<f64>>) -> (Vec<usize>, f64) {
        let num_nodes = self.distance_matrix.len();
        let mut rng = thread_rng();
        let mut tour = Vec::with_capacity(num_nodes + 1);
        let mut visited = vec![false; num_nodes];

        let start_node = rng.gen_range(0..num_nodes);
        tour.push(start_node);
        visited[start_node] = true;
        let mut current_node = start_node;
        let mut total_length = 0.0;

        while tour.len() < num_nodes {
            let probabilities = self.calculate_probabilities(
                current_node,
                &visited,
                pheromones,
            );
            let next_node = self.select_next_node(&probabilities, &mut rng);
            tour.push(next_node);
            visited[next_node] = true;
            total_length += self.distance_matrix[current_node][next_node];
            current_node = next_node;
        }

        // Return to start node
        tour.push(start_node);
        total_length += self.distance_matrix[current_node][start_node];

        (tour, total_length)
    }

    fn calculate_probabilities(
        &self,
        current_node: usize,
        visited: &Vec<bool>,
        pheromones: &Vec<Vec<f64>>,
    ) -> Vec<(usize, f64)> {
        let mut probabilities = Vec::new();
        let mut sum = 0.0;

        for j in 0..self.distance_matrix.len() {
            if !visited[j] {
                let pheromone = pheromones[current_node][j].powf(self.alpha);
                let heuristic = (1.0 / self.distance_matrix[current_node][j]).powf(self.beta);
                let prob = pheromone * heuristic;
                probabilities.push((j, prob));
                sum += prob;
            }
        }

        // Normalize probabilities
        for prob in probabilities.iter_mut() {
            prob.1 /= sum;
        }

        probabilities
    }

    fn select_next_node(
        &self,
        probabilities: &Vec<(usize, f64)>,
        rng: &mut ThreadRng,
    ) -> usize {
        let mut cumulative_prob = 0.0;
        let pick = rng.gen::<f64>();
        for &(node, prob) in probabilities {
            cumulative_prob += prob;
            if pick <= cumulative_prob {
                return node;
            }
        }
        probabilities.last().unwrap().0
    }
}
