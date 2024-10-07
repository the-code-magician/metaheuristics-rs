pub trait Archive {
    type Solution;
    type Fitness;

    fn add(&mut self, solution: Self::Solution, fitness: Self::Fitness);
    fn get_best(&self) -> Option<&Self::Solution>;
}

pub struct BasicArchive<S, F> {
    capacity: usize,
    entries: Vec<(S, F)>,
}

impl<S, F> BasicArchive<S, F>
where
    F: PartialOrd,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::new(),
        }
    }
}

impl<S, F> Archive for BasicArchive<S, F>
where
    F: PartialOrd,
{
    type Solution = S;
    type Fitness = F;

    fn add(&mut self, solution: S, fitness: F) {
        self.entries.push((solution, fitness));
        self.entries.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        if self.entries.len() > self.capacity {
            self.entries.pop();
        }
    }

    fn get_best(&self) -> Option<&Self::Solution> {
        self.entries.first().map(|(solution, _)| solution)
    }
}
