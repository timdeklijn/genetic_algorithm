use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Individual {
    dna: DNA,
    fitness: f32,
}

impl Individual {
    fn new(dna: DNA) -> Self {
        let fitness = dna.calc_fitness();
        Individual { dna, fitness }
    }
}

#[derive(Debug, Clone)]
pub struct Population {
    pop: Vec<Individual>,
    genes: Genes,
}
impl Population {
    fn new(genes: Genes) -> Self {
        let mut pop = Vec::new();
        for _ in 0..POP_SIZE {
            let i = Individual::new(genes.create_individual_genome());
            pop.push(i)
        }
        Population { pop, genes }
    }

    fn pop_printer(&self) {
        for individual in self.pop.iter() {
            let s: String = individual.dna.dna.clone().into_iter().collect();
            if s == ANSWER {
                println!("-- {}", s)
            }
        }
    }

    fn get_max_fitness(&self) -> f32 {
        let fitness_vec = self.get_fitness_vector();
        fitness_vec.iter().fold(-f32::INFINITY, |a, &b| a.max(b))
    }

    fn get_fitness_vector(&self) -> Vec<f32> {
        self.pop.iter().map(|x| x.fitness.clone()).collect()
    }

    fn calc_probability_vector(&self) -> Vec<usize> {
        // Check if l has only zeros
        let fitness_vec = self.get_fitness_vector();
        let all_zero: f32 = fitness_vec
            .iter()
            .filter(|&x| x.clone() != 0.0)
            .fold(0.0, |a, b| a + b);
        // If l has only zeros, make them only 0.1's
        let fitness_vec: Vec<f32> = if all_zero == 0.0 {
            fitness_vec.clone().iter().map(|_| 0.1).collect()
        } else {
            fitness_vec.clone()
        };

        // Find smallest value in the list (ignoring the zeros)
        let min_value = fitness_vec
            .iter()
            .filter(|a| a.fract() != 0.0) // remove zeros
            .fold(f32::INFINITY, |a, &b| a.min(b)); // find minimum
        let factor = (1.0 / min_value).ceil();

        // Fill a list with indices, based on their probability
        let mut p_vec = Vec::new();
        for i in 0..fitness_vec.len() {
            if fitness_vec[i] > 0.0 {
                let n = (fitness_vec[i] * factor).floor() as usize;
                for _ in 0..n {
                    p_vec.push(i)
                }
            }
        }
        p_vec
    }

    fn choose_parents_index(&self, p_vec: &Vec<usize>) -> (usize, usize) {
        // Choose a parent index from the probability vector
        let p1: usize = match p_vec.choose(&mut rand::thread_rng()) {
            Some(x) => x.clone(),
            None => panic!(),
        };
        // If there is only 1 parent, p1 is returned twice
        if p_vec.len() == 1 {
            return (p1, p1);
        }
        // Pick a parent until p2 is not equal to p1
        let p2: usize = loop {
            let pp = match p_vec.choose(&mut rand::thread_rng()) {
                Some(x) => x.clone(),
                None => panic!(),
            };
            // Break the loop if pp is not equal to p1
            if pp != p1 {
                break pp;
            }
        };
        (p1, p2)
    }

    fn new_dna(&self, p1: usize, p2: usize) -> Vec<char> {
        let p1 = self.pop[p1].dna.dna.clone(); //TODO: Ugly
        let p2 = self.pop[p2].dna.dna.clone();

        let mut rng = rand::thread_rng();
        let mut new_dna = Vec::new();
        for i in 0..p1.len() {
            new_dna.push(if rng.gen::<f64>() >= 0.5 {
                p1[i].clone()
            } else {
                p2[i].clone()
            })
        }
        let mut mutated_dna = Vec::new();
        for i in 0..p1.len() {
            if rng.gen::<f64>() < MUTATION_RATE {
                mutated_dna.push(self.genes.genes.choose(&mut rng).unwrap().clone())
            } else {
                mutated_dna.push(new_dna[i])
            }
        }
        mutated_dna
    }

    fn evolve(&mut self) -> Vec<Individual> {
        let mut new_pop = Vec::new();
        let p_vec = self.calc_probability_vector();
        for _ in 0..POP_SIZE {
            let (p1, p2) = self.choose_parents_index(&p_vec);
            let new_dna = self.new_dna(p1, p2);
            let dna = DNA::new(new_dna);
            new_pop.push(Individual::new(dna));
        }
        new_pop
    }

    fn run(&mut self) {
        for g in 0..GENERATIONS {
            self.pop = self.evolve();
            let max_fitness = self.get_max_fitness();
            if g % 100 == 0 || max_fitness == 1.0 {
                println!("{}: {}", g, max_fitness);
                self.pop_printer();
                if max_fitness == 1.0 {
                    break;
                }
            }
        }
    }
}
