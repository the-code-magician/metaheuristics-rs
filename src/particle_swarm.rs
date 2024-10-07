use rand::prelude::*;
use crate::optimizer::Optimizer;
use crate::archive::Archive;

pub struct ParticleSwarm {
    pub swarm_size: usize,
    pub dimensions: usize,
    pub iterations: usize,
    pub inertia_weight: f64,
    pub cognitive_coeff: f64,
    pub social_coeff: f64,
}

impl ParticleSwarm {
    pub fn new(
        swarm_size: usize,
        dimensions: usize,
        iterations: usize,
        inertia_weight: f64,
        cognitive_coeff: f64,
        social_coeff: f64,
    ) -> Self {
        Self {
            swarm_size,
            dimensions,
            iterations,
            inertia_weight,
            cognitive_coeff,
            social_coeff,
        }
    }
}

impl Optimizer for ParticleSwarm {
    type Solution = Vec<f64>;
    type Fitness = f64;

    fn optimize<F, A>(&self, fitness_function: F, archive: &mut A)
    where
        F: Fn(&Self::Solution) -> Self::Fitness,
        A: Archive<Solution=Self::Solution, Fitness=Self::Fitness>,
    {
        let mut rng = thread_rng();

        let mut particles = Vec::with_capacity(self.swarm_size);
        let mut velocities = Vec::with_capacity(self.swarm_size);
        let mut personal_best_positions = Vec::with_capacity(self.swarm_size);
        let mut personal_best_scores = Vec::with_capacity(self.swarm_size);

        let mut global_best_position = vec![0.0; self.dimensions];
        let mut global_best_score = std::f64::INFINITY;

        for _ in 0..self.swarm_size {
            let position: Vec<f64> = (0..self.dimensions)
                .map(|_| rng.gen_range(-10.0..10.0))
                .collect();
            let velocity: Vec<f64> = (0..self.dimensions)
                .map(|_| rng.gen_range(-1.0..1.0))
                .collect();
            let score = fitness_function(&position);

            particles.push(position.clone());
            velocities.push(velocity);
            personal_best_positions.push(position.clone());
            personal_best_scores.push(score);

            archive.add(position.clone(), score);

            if score < global_best_score {
                global_best_score = score;
                global_best_position = position;
            }
        }

        for _ in 0..self.iterations {
            for i in 0..self.swarm_size {
                for d in 0..self.dimensions {
                    velocities[i][d] = self.inertia_weight * velocities[i][d]
                        + self.cognitive_coeff * rng.gen::<f64>()
                            * (personal_best_positions[i][d] - particles[i][d])
                        + self.social_coeff * rng.gen::<f64>()
                            * (global_best_position[d] - particles[i][d]);
                }

                for d in 0..self.dimensions {
                    particles[i][d] += velocities[i][d];
                }

                let score = fitness_function(&particles[i]);

                archive.add(particles[i].clone(), score);

                if score < personal_best_scores[i] {
                    personal_best_scores[i] = score;
                    personal_best_positions[i] = particles[i].clone();
                }

                if score < global_best_score {
                    global_best_score = score;
                    global_best_position = particles[i].clone();
                }
            }
        }
    }
}
