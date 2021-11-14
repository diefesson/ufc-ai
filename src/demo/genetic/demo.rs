use std::time::Instant;

use crate::{
    demo::genetic::*,
    genetic::{evolve, selector::tournament_selector},
};

#[allow(dead_code)]
pub fn demo() {
    let initial_population = vec![-10.0, -3.0, 3.0, 10.0]
        .iter()
        .map(|x| code(*x))
        .collect();

    let selector = tournament_selector(0.5);
    let crosser = crosser(0.6);
    let mutator = mutator(0.01);
    let target_size = 400;
    let generations = 5;

    let start = Instant::now();
    let (final_population, scores) = evolve(
        initial_population,
        &scorer,
        &selector,
        &crosser,
        &mutator,
        target_size,
        generations,
    );
    let end = Instant::now();
    let duration = end - start;

    let (best_index, _) = scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    let best_chromosome = final_population[best_index];
    println!(
        "problema resolvido em {:.2} segundos",
        duration.as_secs_f64()
    );
    println!(
        "cromosomo: {:b} de valor {}",
        best_chromosome,
        decode(best_chromosome)
    );
}
