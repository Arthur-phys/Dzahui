pub mod dim1_time_independent;

pub use dim1_time_independent::StokesParams1D;
pub use dim1_time_independent::StokesSolver1D;

// Aliasing
pub type StaticPressureSolver = StokesSolver1D;
pub type StaticPressureParams = StokesParams1D;
pub type StaticPressureParamsBuilder = StokesParams1DBuilder;

/// Struct to initialize builders params for either time-dependent or time-independent diffussion solvers.
pub struct StokesParams();

#[derive(Default)]
pub struct StokesParams1DBuilder {
    pressure: Option<f64>,
    rho: Option<f64>,
    force_function: Option<Box<dyn Fn(f64) -> f64>>
}


impl StokesParams {
    /// Redirects to 1e Stokes params
    pub fn normal_1d() -> StokesParams1DBuilder {
        StokesParams1DBuilder::default()
    }
    /// Redirects to 1d Stokes params with aliasing
    pub fn static_pressure() -> StaticPressureParamsBuilder {
        StaticPressureParamsBuilder::default()
    }
}

impl StokesParams1DBuilder {
    /// Set pressure
    pub fn hydrostatic_pressure(self, pressure_value: f64) -> Self {
        Self {
            pressure: Some(pressure_value),
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
    /// Build StokesParams1D
    pub fn build(self) -> StokesParams1D {
        
        let hydrostatic_pressure = if let Some(hydrostatic_pressure) = self.pressure {
            hydrostatic_pressure
        } else {
            panic!("Params lack 'hydrostatic_pressure' term!");
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
        
        StokesParams1D {
            hydrostatic_pressure,
            rho,
            force_function
        }
    }
}