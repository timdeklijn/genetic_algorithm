use rand::seq::SliceRandom;
use rand::Rng;

const ANSWER: &str = "To be, or not to be!";
const MUTATION_RATE: f64 = 0.03;

fn pop_printer(population: &Vec<Vec<char>>) {
    for i in population {
        let s: String = i.into_iter().collect();
        if s == ANSWER {
            println!("-- {}", s)
        }
    }
}

fn calculate_fitness(population: &Vec<Vec<char>>) -> Vec<f32> {
    // Claculate fitness
    let mut fitness = Vec::new();
    for i in 0..population.len() {
        // Per DNA sequence check similarity with answer
        let mut f = 0;
        for (i, gene) in population[i].clone().into_iter().enumerate() {
            if gene == ANSWER.chars().nth(i).unwrap() {
                f += 1
            }
        }
        fitness.push(f as f32 / ANSWER.len() as f32);
    }
    fitness
}

fn new_dna(p1: &Vec<char>, p2: &Vec<char>, genes: &Vec<char>) -> Vec<char> {
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
            mutated_dna.push(genes.choose(&mut rng).unwrap().clone())
        } else {
            mutated_dna.push(new_dna[i])
        }
    }
    mutated_dna
}

fn probability_vector(l: &Vec<f32>) -> Vec<usize> {
    // Check if l has only zeros
    let all_zero: f32 = l
        .iter()
        .filter(|&x| x.clone() != 0.0)
        .fold(0.0, |a, b| a + b);
    // If l has only zeros, make them only 0.1's
    let l: Vec<f32> = if all_zero == 0.0 {
        l.clone().iter().map(|_| 0.1).collect()
    } else {
        l.clone()
    };

    // Find smallest value in the list (ignoring the zeros)
    let min_value = l
        .iter()
        .filter(|a| a.fract() != 0.0) // remove zeros
        .fold(f32::INFINITY, |a, &b| a.min(b)); // find minimum
    let factor = (1.0 / min_value).ceil();

    // Fill a list with indices, based on their probability
    let mut p_vec = Vec::new();
    for i in 0..l.len() {
        if l[i] > 0.0 {
            let n = (l[i] * factor).floor() as usize;
            for _ in 0..n {
                p_vec.push(i)
            }
        }
    }
    p_vec
}

fn choose_parents_index(p_vec: Vec<usize>) -> (usize, usize) {
    let p1: usize = match p_vec.choose(&mut rand::thread_rng()) {
        Some(x) => x.clone(),
        None => panic!(),
    };
    if p_vec.len() == 1 {
        return (p1, p1);
    }
    let p2: usize = loop {
        let pp = match p_vec.choose(&mut rand::thread_rng()) {
            Some(x) => x.clone(),
            None => panic!(),
        };
        if pp != p1 {
            break pp;
        }
    };
    (p1, p2)
}

fn main() {
    // Answer Length, used often
    let l = ANSWER.len();

    // Initiate possible genes
    let genes: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ,.?!'; "
        .chars()
        .collect();

    let mut population = Vec::new();
    let pop_size = 100;
    for _ in 0..pop_size {
        // Random sequence of length l picked from genes
        let dna: Vec<char> = genes
            .choose_multiple(&mut rand::thread_rng(), l)
            .cloned()
            .collect();
        population.push(dna)
    }

    for generation in 0..1000 {
        // Claculate fitness
        let fitness = calculate_fitness(&population);

        // Print - and break when needed
        let max_fitness = fitness.iter().fold(-f32::INFINITY, |a, &b| a.max(b));
        if generation % 10 == 0 || max_fitness == 1.0 {
            println!("generation {}: {}", generation, max_fitness);
            pop_printer(&population);
            if max_fitness == 1.0 {
                break;
            }
        }

        // Create new generation
        let mut new_pop = Vec::new();
        let f = fitness.clone();
        for _ in 0..pop_size {
            let p_vec = probability_vector(&f);
            let (p1, p2) = choose_parents_index(p_vec);
            let new_dna = new_dna(&population[p1], &population[p2], &genes);
            new_pop.push(new_dna);
        }
        population = new_pop;
    }
}
