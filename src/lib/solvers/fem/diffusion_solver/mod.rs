// Module declarations
pub mod time_dependent;
pub mod time_independent;

// Internal dependencies + re-exports
pub use time_dependent::{DiffussionParamsTimeDependent, DiffussionSolverTimeDependent};
pub use time_independent::{DiffussionParamsTimeIndependent, DiffussionSolverTimeIndependent};


pub struct DiffussionParams();


impl DiffussionParams {
    pub fn time_dependent() -> DiffussionParamsTimeDependent {
        DiffussionParamsTimeDependent::default()
    }

    pub fn time_independent() -> DiffussionParamsTimeIndependent {
        DiffussionParamsTimeIndependent::default()
    }
}
