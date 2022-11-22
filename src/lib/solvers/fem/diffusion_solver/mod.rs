pub mod time_dependent;
pub mod time_independent;

pub struct DiffussionParams();

#[derive(Debug)]
pub(crate) enum Conditions {
    Uninitialized,
    Are(Vec<f64>)
}

impl Default for Conditions {
    fn default() -> Self {
        Conditions::Uninitialized
    }
}

#[derive(Default,Debug)]
pub struct DiffussionParamsTimeDependent {
    pub mu: f64,
    pub b: f64,
    pub boundary_conditions: [f64;2],
    pub(crate) initial_conditions: Conditions
}

#[derive(Default, Debug)]
pub struct DiffussionParamsTimeIndependent {
    pub mu: f64,
    pub b: f64,
    pub boundary_conditions: [f64;2],
}

impl DiffussionParams {
    pub fn time_dependent() -> DiffussionParamsTimeDependent {
        DiffussionParamsTimeDependent::default()
    }

    pub fn time_independent() -> DiffussionParamsTimeIndependent {
        DiffussionParamsTimeIndependent::default()
    }
}

impl DiffussionParamsTimeDependent {
    pub fn mu(self, mu: f64) -> Self {
        Self {
            mu,
            ..self
        }
    }

    pub fn b(self, b: f64) -> Self {
        Self {
            b,
            ..self
        }
    }

    pub fn boundary_conditions(self, left: f64, right: f64) -> Self {
        Self {
            boundary_conditions: [left, right],
            ..self
        }
    }

    pub fn initial_conditions<A: IntoIterator<Item = f64>>(self, initial_conditions: A) -> Self {
        Self {
            initial_conditions: Conditions::Are(initial_conditions.into_iter().collect()),
            ..self
        }
    }
}

impl DiffussionParamsTimeIndependent {
    pub fn mu(self, mu: f64) -> Self {
        Self {
            mu,
            ..self
        }
    }

    pub fn b(self, b: f64) -> Self {
        Self {
            b,
            ..self
        }
    }

    pub fn boundary_conditions(self, left: f64, right: f64) -> Self {
        Self {
            boundary_conditions: [left, right],
            ..self
        }
    }
}
