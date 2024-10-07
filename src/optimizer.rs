use crate::archive::Archive;

pub trait Optimizer {
    type Solution;
    type Fitness;

    fn optimize<F, A>(&self, fitness_function: F, archive: &mut A)
    where
        F: Fn(&Self::Solution) -> Self::Fitness,
        A: Archive<Solution = Self::Solution, Fitness = Self::Fitness>;
}
