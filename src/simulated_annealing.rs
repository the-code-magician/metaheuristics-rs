use rand::prelude::*;
use crate::optimizer::Optimizer;
use crate::archive::Archive;
use crate::individual::{Individual, Neighbor, FitnessValue};

pub struct SimulatedAnnealing {
    pub initial_temp: f64,
    pub cooling_rate: f64,
    pub iterations: usize,
}

impl SimulatedAnnealing {
    pub fn new(
        initial_temp: f64,
        cooling_rate: f64,
        iterations: usize,
    ) -> Self {
        Self {
            initial_temp,
            cooling_rate,
            iterations,
        }
    }
}

impl<I> Optimizer<I> for SimulatedAnnealing
where
    I: Individual + Neighbor + Default,
    I::Fitness: PartialOrd + FitnessValue,
{
    fn optimize<A>(&self, archive: &mut A)
    where
        A: Archive<Solution = I, Fitness = I::Fitness>,
    {
        use crate::individual::FitnessValue; // Import the trait for to_f64()
        let mut rng = thread_rng();
        let mut current_state = I::default();
        let mut current_temp = self.initial_temp;

        for _ in 0..self.iterations {
            let neighbor = current_state.neighbor(&mut rng);
            let delta = neighbor.fitness().to_f64() - current_state.fitness().to_f64();

            if delta < 0.0 || rng.gen::<f64>() < (-delta / current_temp).exp() {
                current_state = neighbor;
            }

            archive.add(current_state.clone());

            current_temp *= 1.0 - self.cooling_rate;
        }
    }
}
