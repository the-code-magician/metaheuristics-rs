use rand::prelude::*;
use crate::optimizer::Optimizer;
use crate::archive::Archive;
use crate::individual::{Individual, Crossover, Mutate};

pub struct GeneticAlgorithm {
    pub population_size: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub generations: usize,
}

impl GeneticAlgorithm {
    pub fn new(
        population_size: usize,
        mutation_rate: f64,
        crossover_rate: f64,
        generations: usize,
    ) -> Self {
        Self {
            population_size,
            mutation_rate,
            crossover_rate,
            generations,
        }
    }
}

impl<I> Optimizer<I> for GeneticAlgorithm
where
    I: Individual + Crossover + Mutate,
    I::Fitness: PartialOrd,
{
    fn optimize<A>(&self, archive: &mut A)
    where
        A: Archive<Solution = I, Fitness = I::Fitness>,
    {
        let mut population = self.initialize_population();
        let mut rng = thread_rng();

        for _ in 0..self.generations {
            // Add to archive
            for individual in &population {
                archive.add(individual.clone());
            }

            let fitness_scores: Vec<I::Fitness> = population.iter().map(|ind| ind.fitness()).collect();

            let mating_pool = self.selection(&population, &fitness_scores);

            population = self.crossover_and_mutate(mating_pool, &mut rng);
        }
    }
}

impl GeneticAlgorithm {
    fn initialize_population<I>(&self) -> Vec<I>
    where
        I: Individual + Default,
    {
        (0..self.population_size)
            .map(|_| I::default())
            .collect()
    }

    fn selection<I>(&self, population: &Vec<I>, fitness_scores: &Vec<I::Fitness>) -> Vec<I>
    where
        I: Individual + Clone,
    {
        let total_fitness: f64 = fitness_scores
            .iter()
            .map(|f| f.to_f64())
            .sum();

        let mut selected = Vec::with_capacity(self.population_size);
        let mut rng = thread_rng();
        for _ in 0..self.population_size {
            let mut pick = rng.gen::<f64>() * total_fitness;
            let mut current = 0.0;

            for (individual, fitness) in population.iter().zip(fitness_scores.iter()) {
                current += fitness.to_f64();
                if current >= pick {
                    selected.push(individual.clone());
                    break;
                }
            }
        }
        selected
    }

    fn crossover_and_mutate<I>(
        &self,
        mating_pool: Vec<I>,
        rng: &mut ThreadRng,
    ) -> Vec<I>
    where
        I: Individual + Crossover + Mutate,
    {
        let mut new_population = Vec::with_capacity(self.population_size);

        for _ in 0..(self.population_size / 2) {
            let parent1 = &mating_pool[rng.gen_range(0..self.population_size)];
            let parent2 = &mating_pool[rng.gen_range(0..self.population_size)];

            let child1 = if rng.gen::<f64>() < self.crossover_rate {
                parent1.crossover(parent2, rng)
            } else {
                parent1.clone()
            };

            let child2 = if rng.gen::<f64>() < self.crossover_rate {
                parent2.crossover(parent1, rng)
            } else {
                parent2.clone()
            };

            let mut child1 = child1;
            child1.mutate(rng);

            let mut child2 = child2;
            child2.mutate(rng);

            new_population.push(child1);
            new_population.push(child2);
        }

        new_population
    }
}
