pub trait Archive {
    type Solution: crate::Individual;
    type Fitness: PartialOrd;

    fn add(&mut self, solution: Self::Solution);
    fn get_best(&self) -> Option<&Self::Solution>;
}

pub struct BasicArchive<I>
where
    I: crate::Individual,
{
    capacity: usize,
    entries: Vec<I>,
}

impl<I> BasicArchive<I>
where
    I: crate::Individual,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::new(),
        }
    }
}

impl<I> Archive for BasicArchive<I>
where
    I: crate::Individual,
{
    type Solution = I;
    type Fitness = I::Fitness;

    fn add(&mut self, solution: I) {
        self.entries.push(solution);
        self.entries.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());
        if self.entries.len() > self.capacity {
            self.entries.pop();
        }
    }

    fn get_best(&self) -> Option<&Self::Solution> {
        self.entries.first()
    }
}
