
// Internal dependencies

// External dependencies
use ndarray::{Array1, Array2};
use std::fmt::Debug;

/// # General Information
/// 
/// Stokes params in 2D. Equation not yet implemented
/// 
/// # Fields
/// 
/// Not repetead since they're the same as `StokesParams2DBuilder`
/// 
pub struct StokesParams2D {
    pub boundary_conditions: Vec<[f64;2]>,
    pub hydrostatic_pressure: f64,
    pub force_function: Box<dyn Fn([f64;2]) -> [f64;2]>,
    pub rho: f64,
    pub nu: f64
}

impl Default for StokesParams2D {
    fn default() -> Self {
        Self {
            boundary_conditions: vec![],
            hydrostatic_pressure: 0_f64,
            force_function: Box::new(|_| [0_f64;2]),
            nu: 0_f64,
            rho: 0_f64
        }
    }
}

impl Debug for StokesParams2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ff = &self.force_function;
        let eval = ff([0_f64,0_f64]);
        let content = format!("{{ rho: {},\nhydrostatic_pressure: {},\nnu: {},\nboundary_conditions: {:?},\n force_function: f(0,0) -> {:?} }}", self.rho, self.hydrostatic_pressure,self.nu,self.boundary_conditions,eval);
        write!(f, "{}", content)
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct StokesSolver2D {
    pub(crate) stiffness_matrix: Array2<f64>,
    pub(crate) b_vector: Array1<f64>,
    pub boundary_conditions: Vec<f64>,
    pub hydrostatic_pressure: f64,
    pub gauss_step: usize,
    pub rho: f64,
    pub nu: f64,
}