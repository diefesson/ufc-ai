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
    Se: Fn(Vec<S>, Vec<f64>, &C) -> Vec<S>,
    V: FnMut(&Vec<S>, &Vec<f64>) -> bool,
    C: Fn(&S, &S) -> S,
    M: Fn(S) -> S,
{
    let mut scores = population.iter().map(|s| scorer(s)).collect::<Vec<_>>();
    loop {
        population = selector(population, scores, crosser)
            .into_iter()
            .map(mutator)
            .collect();
        scores = population.iter().map(|s| scorer(s)).collect::<Vec<_>>();
        if verifier(&population, &scores) {
            return (population, scores);
        }
    }
}