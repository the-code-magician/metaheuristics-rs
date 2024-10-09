use crate::observer::Observer;
use crate::individual::Individual;

pub struct LoggingObserver<I>
where
    I: Individual,
{
    pub log_iteration: bool,
    pub log_fitness: bool,
    pub log_individuals: bool,
}

impl<I> LoggingObserver<I>
where
    I: Individual,
{
    pub fn new(log_iteration: bool, log_fitness: bool, log_individuals: bool) -> Self {
        Self {
            log_iteration,
            log_fitness,
            log_individuals,
        }
    }
}

impl<I> Observer<I> for LoggingObserver<I>
where
    I: Individual + std::fmt::Debug,
    I::Fitness: std::fmt::Debug,
{
    fn on_start(&mut self) {
        println!("Optimization started.");
    }

    fn on_iteration(&mut self, iteration: usize, population: &[I]) {
        if self.log_iteration {
            println!("Iteration {}", iteration);
        }

        if self.log_fitness {
            let fitness_values: Vec<_> = population.iter().map(|ind| ind.fitness()).collect();
            println!("Fitness values: {:?}", fitness_values);
        }

        if self.log_individuals {
            println!("Population:");
            for individual in population {
                println!("{:?}", individual);
            }
        }
    }

    fn on_finish(&mut self) {
        println!("Optimization finished.");
    }
}