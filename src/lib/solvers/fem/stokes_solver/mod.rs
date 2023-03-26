pub mod dim1;
pub mod dim2;

pub use dim1::StokesParams1D;
pub use dim1::StokesSolver1D;
pub use dim2::StokesParams2D;

// Aliasing
pub type StaticPressureSolver = StokesSolver1D;
pub type StaticPressureParams = StokesParams1D;
pub type StaticPressureParamsBuilder = StokesParams1DBuilder;

/// Struct to initialize builders params for either time-dependent or time-independent diffussion solvers.
pub struct StokesParams();

#[derive(Default)]
/// # General Information
/// 
/// Builder for stokes params in 1D
/// 
/// # Fields
/// 
/// * `hydrostatic_pressure` - hydrostatic pressure
/// * `rho` - density
/// * `force_function` - force applied on the fluid
/// 
pub struct StokesParams1DBuilder {
    hydrostatic_pressure: Option<f64>,
    rho: Option<f64>,
    force_function: Option<Box<dyn Fn(f64) -> f64>>
}

#[derive(Default)]
/// # General Information
/// 
/// Builder for stokes params in 2D. Equation not implemented
/// 
/// # Fields
/// 
/// * `boundary_condtions` - conditions on 2D boundary
/// * `hydrostatic_pressure` - hydrostatic pressure
/// * `force_function` - force applied on the fluid
/// * `rho` - density
/// * `nu` - viscosity
/// 
pub struct StokesParams2DBuilder {
    boundary_conditions: Option<Vec<[f64;2]>>,
    hydrostatic_pressure: Option<f64>,
    force_function: Option<Box<dyn Fn([f64;2]) -> [f64;2]>>,
    rho: Option<f64>,
    nu: Option<f64>
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
    /// Redirects to 2d stokes params
    pub fn normal_2d() -> StokesParams2DBuilder {
        StokesParams2DBuilder::default()
    }
}

impl StokesParams1DBuilder {
    /// Set pressure
    pub fn hydrostatic_pressure(self, pressure_value: f64) -> Self {
        Self {
            hydrostatic_pressure: Some(pressure_value),
            ..self
        }
    }
    /// Set rho
    pub fn density(self, rho: f64) -> Self {
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
        
        let hydrostatic_pressure = if let Some(hydrostatic_pressure) = self.hydrostatic_pressure {
            hydrostatic_pressure
        } else {
            panic!("Params lack 'pressure' term!");
        };

        let rho = if let Some(rho) = self.rho {
            rho
        } else {
            panic!("Params lack 'density' term!");
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

impl StokesParams2DBuilder {
    /// Set boundary conditions
    pub fn boundary_conditions(self, boundary_conditions: Vec<[f64;2]>) -> Self {
        Self {
            boundary_conditions: Some(boundary_conditions),
            ..self
        }
    }
    /// Set pressure
    pub fn pressure(self, pressure_value: f64) -> Self {
        Self {
            hydrostatic_pressure: Some(pressure_value),
            ..self
        }
    }
    /// Set nu
    pub fn kinematic_viscosity(self, viscosity_value: f64) -> Self {
        Self {
            nu: Some(viscosity_value),
            ..self
        }
    }
    /// Set density
    pub fn density(self, density_value: f64) -> Self {
        Self {
            rho: Some(density_value),
            ..self
        }
    }
    /// Set force function
    pub fn force_function(self, func: Box<dyn Fn([f64;2]) -> [f64;2]>) -> Self {
        Self {
            force_function: Some(func),
            ..self
        }
    }
    /// Build params
    pub fn build(self) -> StokesParams2D {
        
        let hydrostatic_pressure = if let Some(p) = self.hydrostatic_pressure {
            p
        } else {
            panic!("Params lack 'pressure' term!");
        };

        let nu = if let Some(n) = self.nu {
            n
        } else {
            panic!("Params lack 'kinematic viscosity' term!");
        };
        
        let force_function = if let Some(f) = self.force_function {
            f
        } else {
            panic!("Params lack 'force_function!");
        };

        let boundary_conditions = if let Some(b) = self.boundary_conditions {
            b
        } else {
            panic!("Params lack 'boundary_conditions'!");
        };

        let rho = if let Some(r) = self.rho {
            r
        } else {
            panic!("Params lack 'density' term!");
        };

        StokesParams2D {
            hydrostatic_pressure,
            nu,
            force_function,
            boundary_conditions,
            rho
        }
    }
}