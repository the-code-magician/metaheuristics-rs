use crate::observer::Observer;
use crate::individual::Individual;

pub struct DistributionObserver<I>
where
    I: Individual,
{
    pub distributions: Vec<Vec<I>>,
}

impl<I> DistributionObserver<I>
where
    I: Individual + Clone,
{
    pub fn new() -> Self {
        Self {
            distributions: Vec::new(),
        }
    }
}

impl<I> Observer<I> for DistributionObserver<I>
where
    I: Individual + Clone,
{
    fn on_start(&mut self) {}

    fn on_iteration(&mut self, _iteration: usize, population: &[I]) {
        self.distributions.push(population.to_vec());
    }

    fn on_finish(&mut self) {}
}
