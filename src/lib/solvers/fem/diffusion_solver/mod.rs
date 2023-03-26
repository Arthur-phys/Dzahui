// Module declarations
pub mod time_dependent;
pub mod time_independent;

// Internal dependencies + re-exports
pub use time_dependent::{DiffussionParamsTimeDependent, DiffussionSolverTimeDependent};
pub use time_independent::{DiffussionParamsTimeIndependent, DiffussionSolverTimeIndependent};


/// Struct to initialize builders params for either time-dependent or time-independent diffussion solvers.
pub struct DiffussionParams();

#[derive(Default)]
/// # General Information
/// 
/// Builder for diffussion params in 1D with time-dependance
/// 
/// # Fields
/// 
/// * `mu` - Movement term
/// * `b` - Velocity term
/// * `boundary_conditions` - Dirichlet conditions
/// * `initial_conditions` - Internal initial conditions
/// 
pub struct DiffussionParamsTimeDependentBuilder {
    mu: Option<f64>,
    b: Option<f64>,
    boundary_conditions: Option<[f64;2]>,
    initial_conditions: Option<Vec<f64>>,
}

#[derive(Default)]
/// # General Information
/// 
/// Builder for diffussion params in 1D
/// 
/// # Fields
/// 
/// * `mu` - Movement term
/// * `b` - Velocity term
/// * `boundary_conditions` - Dirichlet conditions
/// 
pub struct DiffussionParamsTimeIndependentBuilder {
    mu: Option<f64>,
    b: Option<f64>,
    boundary_conditions: Option<[f64;2]>,
}


impl DiffussionParams {
    pub fn time_dependent() -> DiffussionParamsTimeDependentBuilder {
        DiffussionParamsTimeDependentBuilder::default()
    }

    pub fn time_independent() -> DiffussionParamsTimeIndependentBuilder {
        DiffussionParamsTimeIndependentBuilder::default()
    }
}

impl DiffussionParamsTimeDependentBuilder {
    /// Set b
    pub fn b(self, b: f64) -> Self {
        Self {
            b: Some(b),
            ..self
        }
    }
    /// Set mu
    pub fn mu(self, mu: f64) -> Self {
        Self {
            mu: Some(mu),
            ..self
        }
    }
    /// Set boundary conditions
    pub fn boundary_conditions(self, left: f64, right: f64) -> Self {
        Self {
            boundary_conditions: Some([left, right]),
            ..self
        }
    }
    /// Set initial conditions - basic
    pub fn initial_conditions<A: IntoIterator<Item = f64>>(self, initial_conditions: A) -> Self {
        Self {
            initial_conditions: Some(initial_conditions.into_iter().collect()),
            ..self
        }
    }
    /// Use function 
    pub fn initial_conditions_from_function<A: Fn(f64) -> f64, B: AsRef<str>>(_func: A, _mesh: B) -> Self {

        

        todo!()
    }
    /// Build DiffussionParams
    pub fn build(self) -> DiffussionParamsTimeDependent {
        
        let mu = if let Some(mu) = self.mu {
            mu
        } else {
            panic!("Params lack 'mu' term!");
        };

        let b = if let Some(b) = self.b {
            b
        } else {
            panic!("Params lack 'b' term!");
        };

        let boundary_conditions = if let Some(boundary) = self.boundary_conditions {
            boundary
        } else {
            panic!("Params lack boundary conditions!");
        };

        let initial_conditions = if let Some(initial) = self.initial_conditions {
            initial
        } else {
            panic!("Params lack initial conditions!");
        };
        
        DiffussionParamsTimeDependent {
            mu,
            boundary_conditions,
            b,
            initial_conditions
        }
    }
}

impl DiffussionParamsTimeIndependentBuilder {
    /// Set mu
    pub fn mu(self, mu: f64) -> Self {
        Self {
            mu: Some(mu),
            ..self
        }
    }
    /// Set b
    pub fn b(self, b: f64) -> Self {
        Self {
            b: Some(b),
            ..self
        }
    }
    /// Set boundary cconditions
    pub fn boundary_conditions(self, left: f64, right: f64) -> Self {
        Self {
            boundary_conditions: Some([left, right]),
            ..self
        }
    }
    /// Build DiffussionParams
    pub fn build(self) -> DiffussionParamsTimeIndependent {
        
        let mu = if let Some(mu) = self.mu {
            mu
        } else {
            panic!("Params lack 'mu' term!");
        };

        let b = if let Some(b) = self.b {
            b
        } else {
            panic!("Params lack 'b' term!");
        };

        let boundary_conditions = if let Some(boundary) = self.boundary_conditions {
            boundary
        } else {
            panic!("Params lack boundary conditions!");
        };
        
        DiffussionParamsTimeIndependent {
            mu,
            boundary_conditions,
            b,
        }
    }
}
