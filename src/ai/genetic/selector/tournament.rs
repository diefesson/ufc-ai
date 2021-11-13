use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::ai::data::State;

pub fn tournament_selector<S, C>(
    population: &Vec<S>,
    scores: &Vec<f64>,
    crosser: &C,
    ratio: f64,
) -> Vec<S>
where
    S: State,
    C: Fn(&S, &S) -> S,
{
    let mut offspring = Vec::with_capacity(population.len());
    for _ in 0..population.len() {
        let p0 = select(scores, ratio);
        let p1 = select(scores, ratio);
        let c = crosser(&population[p0], &population[p1]);
        offspring.push(c);
    }
    return offspring;
}

fn select(scores: &Vec<f64>, ratio: f64) -> usize {
    let count = (scores.len() as f64 * ratio).ceil() as usize;
    (0..scores.len())
        .into_iter()
        .collect::<Vec<_>>()
        .choose_multiple(&mut thread_rng(), count)
        .reduce(|a, b| if scores[*a] < scores[*b] { b } else { a })
        .unwrap()
        .clone()
}
