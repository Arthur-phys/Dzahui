use ndarray::{Array1, Array, Ix1, Ix2};

/// # General Information
/// 
/// Represents a way of solving a linear problem of the form **Ax=b** via Cholesky method in which **A** is a square matrix, **b** 
/// is a known vector and **x** is to be found.
/// It needs to implement a solver function that returns the desired result.
///
pub trait CholeskySolver {
    /// # General Information
    /// 
    /// A function that solves a system of equations.
    /// 
    /// # Parameters
    /// 
    /// * &self - An instance of a ODE/PDE solver (which solves a particular equation).
    /// 
    fn solve_cholesky(&self) -> Vec<f64>;
}
/// # General Information
/// 
/// Represents a way of solving a linear problem of the form **Ax=b** via Thomas (tridiagonal) method in which **A** is a square matrix, **b** 
/// is a known vector and **x** is to be found.
/// It needs to implement a solver function that returns the desired result.
///
pub trait ThomasSolver {
    /// # General Information
    /// 
    /// A function that solves a system of equations.
    /// 
    /// # Parameters
    /// 
    /// * &self - An instance of a ODE/PDE solver (which solves a particular equation).
    /// 
    fn solve_by_thomas(matrix: &Array<f64,Ix2>, b: &Array<f64,Ix1>) -> Array1<f64> {

        let mut solution = Array1::from_elem(b.len(), 0_f64);
        
        if b.len() == 1 {
            solution[0] = b[0] / matrix[[0,0]];

        } else if b.len() == 2 {
            let det = 1_f64 / (matrix[[0,0]]*matrix[[1,1]] - matrix[[1,0]]*matrix[[0,1]]);
            solution[0] = det * (matrix[[1,1]] * b[0] - matrix[[0,1]] * b[1]);
            solution[1] = det * (-matrix[[1,0]] * b[0] + matrix[[0,0]] * b[1]);

        } else {
            let mut c = Array1::from_elem(b.len()-1, 0_f64);
            let mut d = Array1::from_elem(b.len(), 0_f64);
            c[0] = matrix[[0,1]] / matrix[[0,0]];
            d[0] = b[0] / matrix[[0,0]];

            for i in 1..b.len()-1 {
                c[i] = matrix[[i,i+1]] / (matrix[[i,i]] - matrix[[i,i-1]] * c[i-1]);
                d[i] = (b[i] - matrix[[i,i-1]] * d[i-1]) / (matrix[[i,i]] - matrix[[i,i-1]] * c[i-1]);
            }

            d[b.len()-1] = (b[b.len()-1] - matrix[[b.len()-1,b.len()-2]] * d[b.len()-2]) / (matrix[[b.len()-1,b.len()-1]] - matrix[[b.len()-1,b.len()-2]] * c[b.len()-2]);

            solution[b.len()-1] = d[b.len()-1];
            
            for i in (0..b.len()-1).rev() {
                solution[i] = d[i] - c[i] * solution[i+1];
            }
        }
        solution
    }
}
/// # General Information
/// 
/// Represents a way of solving a linear problem of the form **Ax=b** via Jacobi method in which **A** is a square matrix, **b** 
/// is a known vector and **x** is to be found.
/// It needs to implement a solver function that returns the desired result.
///
pub trait JacobiSolver {
    /// # General Information
    /// 
    /// A function that solves a system of equations.
    /// 
    /// # Parameters
    /// 
    /// * &self - An instance of a ODE/PDE solver (which solves a particular equation).
    /// 
    fn solve_jacobi(&self) -> Vec<f64>;
}
/// # General Information
/// 
/// Represents a way of solving a linear problem of the form **Ax=b** via Gauss-Seidel method in which **A** is a square matrix, **b** 
/// is a known vector and **x** is to be found.
/// It needs to implement a solver function that returns the desired result.
///
pub trait GaussSeidelSolver {
    /// # General Information
    /// 
    /// A function that solves a system of equations.
    /// 
    /// # Parameters
    /// 
    /// * &self - An instance of a ODE/PDE solver (which solves a particular equation).
    /// 
    fn solve_gauss_seidel(&self) -> Vec<f64>;
}