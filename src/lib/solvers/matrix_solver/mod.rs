// local dependencies
use crate::Error;

// External dependencies
use ndarray::{Array1, Array2, Axis};

/// # General Information
///
/// A function that solves a linear problem of the form **Ax=b** via Thomas (tridiagonal) method in which **A** is a square matrix, **b**
/// is a known vector and **x** is to be found.
///
/// # Parameters
///
/// * `matrix` - A square matrix represented by an Array2.
/// * `b` - A vector result from matrix multiplication Ax = b represented by an Array1.
///
pub fn solve_by_thomas(matrix: &Array2<f64>, b: &Array1<f64>) -> Result<Vec<f64>, Error> {

    if !matrix.is_square() || matrix.len_of(Axis(0)) != b.len() {
        return Err(Error::WrongDims)
    }

    let mut solution = vec![0_f64; b.len()];

    let mut c = Array1::from_elem(b.len() - 1, 0_f64);
    let mut d = Array1::from_elem(b.len(), 0_f64);
    c[0] = matrix[[0, 1]] / matrix[[0, 0]];
    d[0] = b[0] / matrix[[0, 0]];

    for i in 1..b.len() - 1 {
        
        c[i] = matrix[[i, i + 1]] / (matrix[[i, i]] - matrix[[i, i - 1]] * c[i - 1]);
        d[i] = (b[i] - matrix[[i, i - 1]] * d[i - 1])
            / (matrix[[i, i]] - matrix[[i, i - 1]] * c[i - 1]);
    }

    d[b.len() - 1] = (b[b.len() - 1] - matrix[[b.len() - 1, b.len() - 2]] * d[b.len() - 2])
        / (matrix[[b.len() - 1, b.len() - 1]]
            - matrix[[b.len() - 1, b.len() - 2]] * c[b.len() - 2]);

    solution[b.len() - 1] = d[b.len() - 1];

    for i in (0..b.len() - 1).rev() {
        solution[i] = d[i] - c[i] * solution[i + 1];
    }
    
    Ok(solution)
}