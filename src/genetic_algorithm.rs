use rand::prelude::*;
use crate::optimizer::Optimizer;
use crate::archive::Archive;

pub struct GeneticAlgorithm {
    pub population_size: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub generations: usize,
    pub gene_length: usize,
    pub gene_generator: Box<dyn Fn() -> f64>,
}

impl GeneticAlgorithm {
    pub fn new(
        population_size: usize,
        mutation_rate: f64,
        crossover_rate: f64,
        generations: usize,
        gene_length: usize,
        gene_generator: Box<dyn Fn() -> f64>,
    ) -> Self {
        Self {
            population_size,
            mutation_rate,
            crossover_rate,
            generations,
            gene_length,
            gene_generator,
        }
    }
}

impl Optimizer for GeneticAlgorithm {
    type Solution = Vec<f64>;
    type Fitness = f64;

    fn optimize<F, A>(&self, fitness_function: F, archive: &mut A)
    where
        F: Fn(&Self::Solution) -> Self::Fitness,
        A: Archive<Solution = Self::Solution, Fitness = Self::Fitness>,
    {
        let mut population = self.initialize_population();
        let mut rng = thread_rng();

        for _ in 0..self.generations {
            let fitness_scores: Vec<f64> = population.iter().map(|individual| fitness_function(individual)).collect();

            // Add to archive
            for (individual, &fitness) in population.iter().zip(fitness_scores.iter()) {
                archive.add(individual.clone(), fitness);
            }

            let mating_pool = self.selection(&population, &fitness_scores);

            population = self.crossover_and_mutate(mating_pool, &mut rng);
        }
    }
}

impl GeneticAlgorithm {
    fn initialize_population(&self) -> Vec<Vec<f64>> {
        (0..self.population_size)
            .map(|_| (0..self.gene_length).map(|_| (self.gene_generator)()).collect())
            .collect()
    }

    fn selection(&self, population: &Vec<Vec<f64>>, fitness_scores: &Vec<f64>) -> Vec<Vec<f64>> {
        let total_fitness: f64 = fitness_scores.iter().sum();
        let mut selected = Vec::with_capacity(self.population_size);

        let mut rng = thread_rng();
        for _ in 0..self.population_size {
            let mut pick = rng.gen::<f64>() * total_fitness;
            let mut current = 0.0;

            for (individual, &fitness) in population.iter().zip(fitness_scores.iter()) {
                current += fitness;
                if current >= pick {
                    selected.push(individual.clone());
                    break;
                }
            }
        }
        selected
    }

    fn crossover_and_mutate(&self, mating_pool: Vec<Vec<f64>>, rng: &mut ThreadRng) -> Vec<Vec<f64>> {
        let mut new_population = Vec::with_capacity(self.population_size);

        for _ in 0..(self.population_size / 2) {
            let parent1 = &mating_pool[rng.gen_range(0..self.population_size)];
            let parent2 = &mating_pool[rng.gen_range(0..self.population_size)];

            let (child1, child2) = if rng.gen::<f64>() < self.crossover_rate {
                self.crossover(parent1, parent2, rng)
            } else {
                (parent1.clone(), parent2.clone())
            };

            new_population.push(self.mutate(child1, rng));
            new_population.push(self.mutate(child2, rng));
        }

        new_population
    }

    fn crossover(&self, parent1: &Vec<f64>, parent2: &Vec<f64>, rng: &mut ThreadRng) -> (Vec<f64>, Vec<f64>) {
        let crossover_point = rng.gen_range(0..parent1.len());
        let mut child1 = parent1[..crossover_point].to_vec();
        child1.extend_from_slice(&parent2[crossover_point..]);

        let mut child2 = parent2[..crossover_point].to_vec();
        child2.extend_from_slice(&parent1[crossover_point..]);

        (child1, child2)
    }

    fn mutate(&self, mut individual: Vec<f64>, rng: &mut ThreadRng) -> Vec<f64> {
        for gene in &mut individual {
            if rng.gen::<f64>() < self.mutation_rate {
                *gene = (self.gene_generator)();
            }
        }
        individual
    }
}
