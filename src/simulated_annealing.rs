use rand::prelude::*;
use crate::optimizer::Optimizer;
use crate::archive::Archive;

pub struct SimulatedAnnealing {
    pub initial_temp: f64,
    pub cooling_rate: f64,
    pub iterations: usize,
    pub initial_state: Vec<f64>,
    pub neighbor_function: Box<dyn Fn(&Vec<f64>) -> Vec<f64>>,
}

impl SimulatedAnnealing {
    pub fn new(
        initial_temp: f64,
        cooling_rate: f64,
        iterations: usize,
        initial_state: Vec<f64>,
        neighbor_function: Box<dyn Fn(&Vec<f64>) -> Vec<f64>>,
    ) -> Self {
        Self {
            initial_temp,
            cooling_rate,
            iterations,
            initial_state,
            neighbor_function,
        }
    }
}

impl Optimizer for SimulatedAnnealing {
    type Solution = Vec<f64>;
    type Fitness = f64;

    fn optimize<F, A>(&self, fitness_function: F, archive: &mut A)
    where
        F: Fn(&Self::Solution) -> Self::Fitness,
        A: Archive<Solution=Self::Solution, Fitness=Self::Fitness>,
    {
        let mut current_state = self.initial_state.clone();
        let mut current_temp = self.initial_temp;
        let mut rng = thread_rng();

        for _ in 0..self.iterations {
            let neighbor = (self.neighbor_function)(&current_state);
            let delta = fitness_function(&neighbor) - fitness_function(&current_state);

            if delta < 0.0 || rng.gen::<f64>() < (-delta / current_temp).exp() {
                current_state = neighbor;
            }

            let current_fitness = fitness_function(&current_state);
            archive.add(current_state.clone(), current_fitness);

            current_temp *= 1.0 - self.cooling_rate;
        }
    }
}
