# Metaheuristics in Rust

![example branch parameter](https://github.com/the-code-magician/metaheuristics-rs/actions/workflows/rust.yml/badge.svg?branch=feature-1)

## Notice
I am building this repository with ChatGPT as an experiment.

## Introduction

A Rust library implementing various metaheuristic optimization algorithms, including Genetic Algorithms, Particle Swarm Optimization, Simulated Annealing, and Ant Colony Optimization. This library provides a flexible and extensible framework for optimization problems, with support for custom individuals, fitness functions, and observers to monitor the optimization process.

# Metaheuristics Rust Library

A Rust library implementing various metaheuristic optimization algorithms, including Genetic Algorithms, Particle Swarm Optimization, Simulated Annealing, and Ant Colony Optimization. This library provides a flexible and extensible framework for optimization problems, with support for custom individuals, fitness functions, and observers to monitor the optimization process.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Optimization Algorithms](#optimization-algorithms)
  - [Individuals](#individuals)
  - [Observers](#observers)
  - [Archives](#archives)
- [Examples](#examples)
  - [Genetic Algorithm](#genetic-algorithm)
  - [Particle Swarm Optimization](#particle-swarm-optimization)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Multiple Optimization Algorithms**: Implementations of Genetic Algorithms, Particle Swarm Optimization, Simulated Annealing, and Ant Colony Optimization.
- **Flexible Individual Representation**: Support for numeric, bitstring, and custom individuals.
- **Observer Pattern**: An extensible observer system to monitor and log optimization events.
- **Archives**: Keep track of the best solutions found durin

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
metaheuristics = { git = "https://github.com/the-code-magician/metaheuristics-rs" }
```


## Usage
### Optimization Algorithms
The library provides several optimization algorithms through the `Optimizer` trait:

* `GeneticAlgorithm`
* `ParticleSwarm`
* `SimulatedAnnealing`
* `AntColony`

Each optimizer can be customized and used to solve optimization problems by defining appropriate individuals and fitness functions.

### Individuals
Individuals represent candidate solutions and must implement the Individual trait:

```rust
pub trait Individual: Clone {
    type Fitness: FitnessValue;

    fn fitness(&self) -> Self::Fitness;
}

```

Several individual types are provided:

* `NumericIndividual`
* `BitStringIndividual`
* `ParticleIndividual`
* `TourIndividual`

### Observers
Observers monitor the optimization process by handling events emitted by the optimizers. Implement the Observer trait to create custom observers:

```rust
pub trait Observer<I>
where
    I: Individual,
{
    fn emit(&mut self, event: &Event<I>);
}
```


### Archives
Archives store the best solutions found during optimization. Use the Archive trait to define custom archives or use the provided `BasicArchive`:

```rust
pub trait Archive {
    type Solution: Individual;
    type Fitness: FitnessValue;

    fn add(&mut self, solution: Self::Solution);
    fn get_best(&self) -> Option<&Self::Solution>;
}
```

## Examples
### Genetic Algorithm
Here's how to use the `GeneticAlgorithm` optimizer with a numeric individual:

```rust
use metaheuristics::genetic_algorithm::GeneticAlgorithm;
use metaheuristics::optimizer::Optimizer;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::individuals::NumericIndividual;
use metaheuristics::observer::Observer;
use metaheuristics::logging_observer::LoggingObserver;

fn main() {
    // Define the fitness function
    let fitness_function = |genes: &Vec<f64>| -> f64 {
        // Example: Sphere function (minimize sum of squares)
        genes.iter().map(|&x| x * x).sum()
    };

    // Create a genetic algorithm instance
    let ga = GeneticAlgorithm::new(50, 0.05, 0.8, 30);

    // Create an archive to store the best individuals
    let mut archive = BasicArchive::new(5);

    // Create observers
    let mut logging_observer = LoggingObserver;

    // Run the optimization
    ga.optimize(&mut archive, &mut [logging_observer]);

    // Retrieve and display the best individual
    if let Some(best_individual) = archive.get_best() {
        println!("Best individual: {:?}", best_individual);
    }
}
```

### Particle Swarm Optimization
Here's how to use the `ParticleSwarm` optimizer:

```rust
use metaheuristics::particle_swarm::ParticleSwarm;
use metaheuristics::archive::{BasicArchive, Archive};
use metaheuristics::individuals::ParticleIndividual;
use metaheuristics::observer::Observer;

fn main() {
    // Define the fitness function
    let fitness_function = |position: &Vec<f64>| -> f64 {
        // Example: Sphere function
        position.iter().map(|&x| x * x).sum()
    };

    // Create a particle swarm optimizer instance
    let pso = ParticleSwarm::new(
        30,             // swarm_size
        5,              // dimensions
        100,            // iterations
        0.5,            // inertia_weight
        1.5,            // cognitive_coeff
        1.5,            // social_coeff
        fitness_function,
    );

    // Create an archive to store the best particles
    let mut archive = BasicArchive::new(5);

    // Run the optimization
    let mut observers = []; // Use an empty array if you have no observers
    pso.optimize(&mut archive, &mut observers);

    // Retrieve and display the best particle
    if let Some(best_particle) = archive.get_best() {
        println!("Best particle: {:?}", best_particle);
    }
}
```

### Contributing
Contributions are welcome! Please follow these steps:

* Fork the repository.
* Create a new branch with a descriptive name.
* Make your changes, ensuring code quality and consistency.
* Submit a pull request with a detailed description of your changes.
* Please make sure to update or add tests as appropriate.

## License
This project is licensed under the MIT License.

Note: This README provides an overview of the metaheuristics library, usage examples, and guidelines for contributing. For more detailed documentation, please refer to the individual modules and source code comments.
