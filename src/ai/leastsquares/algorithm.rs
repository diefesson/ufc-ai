use nalgebra::*;
use std::convert::TryInto;

type XMatrix = OMatrix<f64, Dynamic, U3>;
type YMatrix = OMatrix<f64, Dynamic, U1>;

pub fn least_squares<const X: usize, const O: usize>(
    x_data: &[[f64; X]],
    y_data: &[f64],
) -> [f64; O] {
    debug_assert!(
        X == O - 1,
        "params must be 1 unit greather than x because c"
    );
    let x = XMatrix::from_fn(
        x_data.len(),
        |r, c| if c == 0 { 1.0 } else { x_data[r][c - 1] },
    );
    let y = YMatrix::from_iterator(y_data.len(), y_data.iter().copied());
    let b = (x.transpose() * &x).try_inverse().unwrap() * x.transpose() * y;
    b.into_iter()
        .copied()
        .collect::<Vec<f64>>()
        .try_into()
        .unwrap()
}
