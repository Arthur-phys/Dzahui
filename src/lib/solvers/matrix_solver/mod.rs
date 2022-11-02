// local dependencies
use crate::Error;

// External dependencies
use ndarray::{Array2, Array1};

/// # General Information
///
/// A function that solves a system of equations using the Cholesky method.
///
/// # Parameters
///
/// * `matrix` - A square matrix represented by an Array2. 
/// * `b` - A vector result from matrix multiplication Ax = b represented by an Array1. 
///
pub fn solve_by_cholesky(_matrix: &Array2<f64>, _b: &Array1<f64>) -> Result<Vec<f64>,Error> {
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

    let mut solution = vec![0_f64; b.len() + 2];//Array1::from_elem(b.len() + 2, 0_f64);

    // Solution for 1x1 Matrix is trivial
    if b.len() == 1 {
        solution[1] = b[0] / matrix[[0, 0]];

    // Solution for 2X2 Matrix is handcrafted since it cannot be a tridiagonal system
    } else if b.len() == 2 {
        let det = 1_f64 / (matrix[[0, 0]] * matrix[[1, 1]] - matrix[[1, 0]] * matrix[[0, 1]]);
        solution[1] = det * (matrix[[1, 1]] * b[0] - matrix[[0, 1]] * b[1]);
        solution[2] = det * (-matrix[[1, 0]] * b[0] + matrix[[0, 0]] * b[1]);

        // Any bigger squared matrix is solved normally via the algorithm
    } else {
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

        solution[b.len()] = d[b.len() - 1];

        for i in (0..b.len() - 1).rev() {
            solution[i + 1] = d[i] - c[i] * solution[i + 2];
        }
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
pub fn solve_by_jacobi(_matrix: &Array2<f64>, _b: &Array1<f64>) -> Result<Vec<f64>,Error> {
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
pub fn solve_by_gauss_seidel(_matrix: &Array2<f64>, _b: &Array1<f64>) -> Result<Vec<f64>,Error> {
    todo!();
}