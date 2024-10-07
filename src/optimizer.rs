pub trait Optimizer {
    type Solution;
    type Fitness;

    fn optimize<F>(&self, fitness_function: F) -> Self::Solution
    where
        F: Fn(&Self::Solution) -> Self::Fitness;
}
