use nalgebra::DMatrix;
use std::convert::TryInto;

pub fn least_squares<const X: usize, const O: usize>(
    x_data: &[[f64; X]],
    y_data: &[f64],
    lambda: f64,
) -> [f64; O] {
    debug_assert_eq!(
        X,
        O - 1,
        "output vector len should be 1 unit greater than x vectors len"
    );
    assert_eq!(x_data.len(), y_data.len());

    let n = x_data.len();
    let x = DMatrix::from_fn(n, O, |r, c| if c == 0 { 1.0 } else { x_data[r][c - 1] });
    let y = DMatrix::from_iterator(n, 1, y_data.iter().copied());
    let r = DMatrix::from_diagonal_element(O, O, lambda);
    let p = (x.transpose() * &x + r).try_inverse().unwrap() * x.transpose();
    let b = p * y;
    b.into_iter()
        .copied()
        .collect::<Vec<f64>>()
        .try_into()
        .unwrap()
}
