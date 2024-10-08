use crate::archive::Archive;

pub trait Optimizer<I>
where
    I: crate::Individual,
{
    fn optimize<A>(&self, archive: &mut A)
    where
        A: Archive<Solution = I, Fitness = I::Fitness>;
}
