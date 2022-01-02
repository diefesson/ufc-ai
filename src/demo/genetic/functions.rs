use rand::{thread_rng, Rng};

pub fn crosser(cross_point: f64) -> impl Fn(&u32, &u32) -> u32 {
    let mask = create_mask(cross_point);
    move |parent_0, parent_1| (parent_0 & mask) | (parent_1 & !mask)
}

fn create_mask(cross_point: f64) -> u32 {
    let cross_point = (u32::BITS as f64 * cross_point) as usize;
    let mut mask = 0;
    for i in 0..cross_point {
        mask = mask | (1 << i);
    }
    return mask;
}

pub fn mutator(rate: f64) -> impl Fn(&u32) -> u32 {
    move |chromosome| {
        let mut rng = thread_rng();
        let mut mutated = *chromosome;
        for i in 0..u32::BITS {
            if rng.gen_bool(rate) {
                let mask: u32 = 1 << i;
                mutated = mutated ^ mask;
            }
        }
        return mutated;
    }
}

pub fn scorer(chromosome: &u32) -> f64 {
    let x = decode(*chromosome);
    -(x * x - 3.0 * x + 4.0)
}

pub fn code(x: f64) -> u32 {
    let x_from = -10.0;
    let x_to = 10.0;
    let y_from = u32::MIN as f64;
    let y_to = u32::MAX as f64;
    let y = y_from + (x - x_from) * ((y_to - y_from) / (x_to - x_from));
    return y as u32;
}

pub fn decode(x: u32) -> f64 {
    let x = x as f64;
    let x_from = u32::MIN as f64;
    let x_to = u32::MAX as f64;
    let y_from = -10.0;
    let y_to = 10.0;
    return y_from + (x - x_from) * ((y_to - y_from) / (x_to - x_from));
}
