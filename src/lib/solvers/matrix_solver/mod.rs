// local dependencies
use crate::Error;

// External dependencies
use ndarray::{Array1, Array2};

/// # General Information
///
/// A function that solves a system of equations using the Cholesky method.
///
/// # Parameters
///
/// * `matrix` - A square matrix represented by an Array2.
/// * `b` - A vector result from matrix multiplication Ax = b represented by an Array1.
///
pub fn solve_by_cholesky(_matrix: &Array2<f64>, _b: &Array1<f64>) -> Result<Vec<f64>, Error> {
    todo!();
}
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

/// # General Information
///
/// A function that solves a system of equations using the Jacobi method.
///
/// # Parameters
///
/// * `matrix` - A square matrix represented by an Array2.
/// * `b` - A vector result from matrix multiplication Ax = b represented by an Array1.
///
pub fn solve_by_jacobi(_matrix: &Array2<f64>, _b: &Array1<f64>) -> Result<Vec<f64>, Error> {
    todo!();
}

/// # General Information
///
/// A function that solves a system of equations via the Gauss-Seidel method.
///
/// # Parameters
///
/// * `matrix` - A square matrix represented by an Array2.
/// * `b` - A vector result from matrix multiplication Ax = b represented by an Array1.
///
pub fn solve_by_gauss_seidel(_matrix: &Array2<f64>, _b: &Array1<f64>) -> Result<Vec<f64>, Error> {
    todo!();
}
