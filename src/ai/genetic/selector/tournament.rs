use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn tournament_selector(ratio: f64) -> impl Fn(&Vec<f64>) -> usize {
    move |scores| {
        let count = (scores.len() as f64 * ratio).ceil() as usize;
        (0..scores.len())
            .into_iter()
            .collect::<Vec<_>>()
            .choose_multiple(&mut thread_rng(), count)
            .max_by(|a, b| scores[**a].partial_cmp(&scores[**b]).unwrap())
            .unwrap()
            .clone()
    }
}
