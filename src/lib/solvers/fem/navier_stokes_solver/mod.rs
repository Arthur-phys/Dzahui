pub mod dim1_time_independent;
pub mod dim1_time_dependent;

pub use dim1_time_independent::NavierStokesParams1DTimeIndependent;
pub use dim1_time_independent::NavierStokesSolver1DTimeIndependent;

// Aliasing
pub type StaticPressureSolver = NavierStokesSolver1DTimeIndependent;
pub type StaticPressureParams = NavierStokesParams1DTimeIndependent;
pub type StaticPressureParamsBuilder = NavierStokesParams1DTimeIndependentBuilder;

/// Struct to initialize builders params for either time-dependent or time-independent diffussion solvers.
pub struct NavierStokesParams();

#[derive(Default)]
pub struct NavierStokesParams1DTimeIndependentBuilder {
    speed: Option<f64>,
    rho: Option<f64>,
    force_function: Option<Box<dyn Fn(f64) -> f64>>
}


impl NavierStokesParams {
    /// Redirects to time indepentend 1d Navier-Stokes params
    pub fn time_independent1d() -> NavierStokesParams1DTimeIndependentBuilder {
        NavierStokesParams1DTimeIndependentBuilder::default()
    }
    /// Redirects to time indepentend 1d Navier-Stokes params with aliasing
    pub fn static_pressure() -> StaticPressureParamsBuilder {
        StaticPressureParamsBuilder::default()
    }
}

impl NavierStokesParams1DTimeIndependentBuilder {
    /// Set speed
    pub fn speed(self, speed: f64) -> Self {
        Self {
            speed: Some(speed),
            ..self
        }
    }
    /// Set rho
    pub fn rho(self, rho: f64) -> Self {
        Self {
            rho: Some(rho),
            ..self
        }
    }
    /// Set force function 
    pub fn force_function(self, func: Box<dyn Fn(f64) -> f64>) -> Self {
        Self {
            force_function: Some(func),
            ..self
        }
    }
    /// Build NavierStokesParams1D
    pub fn build(self) -> NavierStokesParams1DTimeIndependent {
        
        let speed = if let Some(speed) = self.speed {
            speed
        } else {
            panic!("Params lack 'speed' term!");
        };

        let rho = if let Some(rho) = self.rho {
            rho
        } else {
            panic!("Params lack 'rho' term!");
        };

        let force_function = if let Some(func) = self.force_function {
            func
        } else {
            panic!("Params lack force_function!");
        };
        
        NavierStokesParams1DTimeIndependent {
            speed,
            rho,
            force_function
        }
    }
}