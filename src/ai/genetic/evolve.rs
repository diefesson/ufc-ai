use crate::ai::data::State;

pub fn evolve<S, Sc, Se, C, M, V>(
    mut population: Vec<S>,
    scorer: &Sc,
    selector: &Se,
    crosser: &C,
    mutator: &M,
    verifier: &mut V,
) -> (Vec<S>, Vec<f64>)
where
    S: State,
    Sc: Fn(&S) -> f64,
    Se: Fn(&Vec<f64>) -> usize,
    V: FnMut(&Vec<S>, &Vec<f64>) -> bool,
    C: Fn(&S, &S) -> S,
    M: Fn(S) -> S,
{
    let mut scores = population.iter().map(|s| scorer(s)).collect::<Vec<_>>();
    loop {
        let mut offspring = Vec::with_capacity(population.len());
        for _ in 0..population.len() {
            let parent_0 = &population[selector(&scores)];
            let parent_1 = &population[selector(&scores)];
            let child = mutator(crosser(parent_0, parent_1));
            offspring.push(child);
        }
        population = offspring;
        scores = population.iter().map(|s| scorer(s)).collect::<Vec<_>>();
        if verifier(&population, &scores) {
            return (population, scores);
        }
    }
}
