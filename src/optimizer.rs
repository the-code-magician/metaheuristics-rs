use crate::archive::Archive;
use crate::observer::Observer;

pub trait Optimizer<I>
where
    I: crate::Individual,
{
    fn optimize<A, O>(&self, archive: &mut A, observers: &mut [O])
    where
        A: Archive<Solution = I, Fitness = I::Fitness>,
        O: Observer<I>;
}