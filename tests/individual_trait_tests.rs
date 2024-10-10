use metaheuristics::individual::{Individual, Crossover, Mutate};
use metaheuristics::genetic_algorithm::GeneticAlgorithm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::observer::Observer;
use metaheuristics::archive::{BasicArchive, Archive};
use rand::prelude::*;
use metaheuristics::logging_observers::LoggingObserver;
use metaheuristics::distribution_observers::DistributionObserver;


#[derive(Clone)]
struct TestIndividual {
    data: Vec<u8>,
}

impl Individual for TestIndividual {
    type Fitness = f64;

    fn fitness(&self) -> Self::Fitness {
        // Example fitness function: count number of zeros
        self.data.iter().filter(|&&x| x == 0).count() as f64
    }
}

impl Crossover for TestIndividual {
    fn crossover(&self, other: &Self, rng: &mut ThreadRng) -> Self {
        let point = rng.gen_range(0..self.data.len());
        let mut new_data = self.data[..point].to_vec();
        new_data.extend_from_slice(&other.data[point..]);
        Self { data: new_data }
    }
}

impl Mutate for TestIndividual {
    fn mutate(&mut self, rng: &mut ThreadRng) {
        let idx = rng.gen_range(0..self.data.len());
        self.data[idx] = rng.gen::<u8>();
    }
}

#[test]
fn test_individual_trait_with_genetic_algorithm() {
    let ga = GeneticAlgorithm::new(50, 0.05, 0.8, 30);
    let mut archive: BasicArchive<TestIndividual> = BasicArchive::new(5);
    
    let mut observers: Vec<&mut dyn Observer<TestIndividual>> = vec![
        LoggingObserver::new(true, true, true),
        DistributionObserver::new(),
    ];

    ga.optimize(&mut archive, &mut observers);

    assert!(archive.get_best().is_some());
}
