pub trait Observer<I>
where
    I: crate::Individual,
{
    /// Called at the beginning of the optimization process.
    fn on_start(&mut self);

    /// Called at each iteration with the current population.
    fn on_iteration(&mut self, iteration: usize, population: &[I]);

    /// Called at the end of the optimization process.
    fn on_finish(&mut self);
}
