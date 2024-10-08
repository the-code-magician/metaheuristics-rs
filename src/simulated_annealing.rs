use rand::prelude::*;
use crate::optimizer::Optimizer;
use crate::archive::Archive;
use crate::individual::{Individual, Neighbor, FitnessValue};
use crate::observer::Observer;


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
    fn optimize<A, O>(&self, archive: &mut A, observers: &mut [O])
    where
        A: Archive<Solution = I, Fitness = I::Fitness>,
        O: Observer<I>,
    {
        let mut rng = thread_rng();
        let mut current_state = I::default();
        let mut current_temp = self.initial_temp;

        for observer in observers.iter_mut() {
            observer.on_start();
            observer.on_iteration(0, &[current_state.clone()]);
        }

        for iteration in 1..=self.iterations {
            let neighbor = current_state.neighbor(&mut rng);
            let delta = neighbor.fitness().to_f64() - current_state.fitness().to_f64();

            if delta < 0.0 || rng.gen::<f64>() < (-delta / current_temp).exp() {
                current_state = neighbor;
            }

            archive.add(current_state.clone());

            for observer in observers.iter_mut() {
                observer.on_iteration(iteration, &[current_state.clone()]);
            }

            current_temp *= 1.0 - self.cooling_rate;
        }

        for observer in observers.iter_mut() {
            observer.on_finish();
        }
    }
}