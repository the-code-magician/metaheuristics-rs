pub trait FitnessValue {
    fn to_f64(&self) -> f64;
}

impl FitnessValue for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}

pub trait Individual: Clone {
    type Fitness: PartialOrd + FitnessValue;

    fn fitness(&self) -> Self::Fitness;
}

pub trait Crossover: Individual {
    fn crossover(&self, other: &Self, rng: &mut rand::rngs::ThreadRng) -> Self;
}

pub trait Mutate: Individual {
    fn mutate(&mut self, rng: &mut rand::rngs::ThreadRng);
}

pub trait Neighbor: Individual {
    fn neighbor(&mut self, rng: &mut rand::rngs::ThreadRng) -> Self;
}

pub trait Default: Individual {
    fn default() -> Self;
}

