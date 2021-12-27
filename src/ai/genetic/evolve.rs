pub fn evolve<S, Sc, Se, Cr, M>(
    mut population: Vec<S>,
    scorer: &Sc,
    selector: &Se,
    crosser: &Cr,
    mutator: &M,
    target_size: usize,
    generations: usize,
) -> (Vec<S>, Vec<f64>)
where
    Sc: Fn(&S) -> f64,
    Se: Fn(&Vec<f64>) -> usize,
    Cr: Fn(&S, &S) -> S,
    M: Fn(&S) -> S,
{
    let mut scores = population.iter().map(|s| scorer(s)).collect::<Vec<_>>();
    for _ in 0..generations {
        let mut offspring = Vec::with_capacity(population.len());
        for _ in 0..target_size {
            let parent_0 = &population[selector(&scores)];
            let parent_1 = &population[selector(&scores)];
            let child = mutator(&crosser(parent_0, parent_1));
            offspring.push(child);
        }
        population = offspring;
        scores = population.iter().map(|s| scorer(s)).collect::<Vec<_>>();
    }
    return (population, scores);
}
