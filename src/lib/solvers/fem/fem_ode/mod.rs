pub mod diffusion_solver;
mod quadrature;
mod linear_solver;

use super::Vertex;

/// # General Information
/// 
/// A struct that implements DiffEquationSolver is implied to represent all needed information for a certain ODE to be solved. Therefore, a function to solve the
/// equation needs to be present in such a structure.
/// This trait **does not** consider time to be within the variables to be solved for. For that case, refer to `TimeDiffEquationSolver`.
/// 
trait DiffEquationSolver {
    
    /// # General Information
    /// 
    /// solve returns a vector of values representing the solution of an equation at a given collection of nodes provided by the user at the creation of an
    /// instance of a solver.
    /// 
    /// # Parameters
    /// 
    fn solve() -> Vec<f32>;
}

/// # General Information
/// 
/// A struct that implements TimeDiffEquationSolver is implied to represent all needed information for a certain time-dependant ODE to be solved.
/// A time dependant ODE needs to have a time-step assigned to the function that is to solve the problem, that's why a delta time is accepted by the main function
/// of the trait.
/// 
trait TimeDiffEquationSolver {
    
    /// # General Information
    /// 
    /// do_step returns a vector of values representing the solution at a given time and collection of nodes provided by the user at the creation of an instance
    /// of a time-dependant solver.
    /// 
    /// # Parameters
    /// 
    /// * `delta_time` - A time step to advance the equation.
    /// 
    fn do_step(delta_time: f32) -> Vec<f32>;
}

enum PolynomialDegree {
    One,
    Two,
    Three,
}

enum Equation {
    Diffusion,
}

struct Vertex1D {
    x: f32
}

impl Clone for Vertex1D {
    fn clone(&self) -> Self {
        Self { x: self.x }
    }
}

impl PartialEq for Vertex1D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl Eq for Vertex1D {}

impl PartialOrd for Vertex1D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x < other.x {
            Some(std::cmp::Ordering::Less)
        } else if self.x == other.x {
            Some(std::cmp::Ordering::Equal)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

impl Ord for Vertex1D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x < other.x {
            std::cmp::Ordering::Less
        } else if self.x == other.x {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl Vertex for Vertex1D {}

struct BoundaryVertex1D {
    id: u32,
    boundary_condition: f32
}

/// # General Information
/// 
/// An empty trait representing the possible lenght of a unit-located piecewise polynomial finite basis. Every array of functions whose length could represent
/// a basis should implement it. That is, a fourth degree piecewise polynomial finite basis, for example, consists of five functions (since the number of nodes
/// required to completely determine a fourth degree polynomial on an interval is five and for each node a function should be created), therefore a trait for this
/// basis should be implemented for `[Box<dyn Fn(f32) -> f32>; 5]` and then on a struct representing such basis, the trait 
/// `FunctionBase1D<[Box<dyn Fn(f32) -> f32>; 5]>` needs to be implemented.
/// 
trait BasisLenght {}

impl BasisLenght for [Box<dyn Fn(f32) -> f32>; 2] {}
impl BasisLenght for [Box<dyn Fn(f32) -> f32>; 3] {}
impl BasisLenght for [Box<dyn Fn(f32) -> f32>; 4] {}

/// # General Information
/// 
/// Represents a set of functions that any piecewise polynomial finite basis should have. Since a basis for a given problem cannot be known beforehand because
/// of interval number and length, most of the functionality of this trait is directed towards generating such a base given an initial set of nodes.
/// 
trait FunctionBase1D<A: BasisLenght> {
    
    /// # General Information
    /// 
    /// A function that returns another one that represents a translation from the unit interval to any other interval given by `start` and `finish`.
    /// 
    /// # Parameters
    /// 
    /// * `start` - Left side of an interval.
    /// * `finish` - Right side of an interval.
    /// 
    fn interval_transformation(start: f32, finish: f32) -> Box<dyn Fn(f32) -> f32> {
        Box::new(move |x: f32| {
            (x - start) / (finish - start)
        })
    }
    
    /// # General Information
    /// 
    /// A way of returning the unit basis the struct holds.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - The struct should provide enough information to generate or hold a unit basis.
    /// 
    fn unit_basis(&self) -> &A;

    /// # General Information
    /// 
    /// A basis mesh differs from the original input mesh in that it may need intermediate points between any two next to each other nodes to fully determine
    /// a basis. This is, in fact, a consequence of the degree of the polynomials of a given basis.
    /// 
    /// # Parameters
    /// 
    /// * `mesh` - A series of vertices representing a partitioned interval.
    /// 
    fn build_basis_mesh(mesh: &Vec<Vertex1D>) -> (Vec<Vertex1D>,u32);
    
    /// # General Information
    /// 
    /// Given a basis mesh, a basis should be built from the original unit basis.
    /// 
    /// # Parameters
    /// 
    /// * `basis_mesh` - A tuple containing a modified mesh and the number of points per interval (of the original mesh). Normally obtained via `build_basis_mesh(..)`.
    /// 
    fn build_basis(basis_mesh: (&Vec<Vertex1D>,f32)) -> Vec<Box<dyn Fn(f32) -> f32>>;
    
    /// # General Information
    /// 
    /// Given a start a finish and a number of intervals, an equidistant basis is generated. Since this process is simpler than giving any mesh, this function both
    /// generates a modified mesh like `build_basis_mesh(..)` and creates the final basis like `build_basis(..)`.
    /// 
    /// # Parameters
    ///  
    /// * `start` - Left side value of an interval (infimum).
    /// * `finish` - Right side value of an interval (maximum).
    /// * `interval_numbner` - Number of  divisions the interval should have.
    /// 
    fn build_equidistant_basis(start: f32, finish: f32, interval_number: u32) -> Vec<Box<dyn Fn(f32) -> f32>>;

}

/// # General Information
/// 
/// A trait that represents the differentiability of a basis. **It's probably to be deprecated in favor of a per-function trait that does the same thing**.
/// When a basis is differentiable, derivatives of it are obtained via chain rule of a transformation and a unit basis. Since both are kwown always, differentiability
/// of a basis is possible without even knowing the original basis.
/// 
trait DifferentiableBasis<A: BasisLenght>: FunctionBase1D<A> {

    /// # General Information
    /// 
    /// Simple derivative of the interval transformation function. Since it's a constant, it does not require to return a function, rather it returns a different
    /// f32 value given an interval.
    /// 
    /// # Parameters
    /// 
    /// * `start` - Left side of an interval (infimum).
    /// * `finish` - Right side of an interval (maximum).
    ///  
    fn interval_transformation_derivative(start: f32, finish: f32) -> f32 {
        1.0 / (finish - start)
    }

    /// # General Information
    /// 
    /// Creates the derivative of a given basis and returns it as a vector of functions. It does not depend on knowing the original basis since chain rule is used.
    /// 
    /// # Parameters
    /// 
    /// * A tuple containing a modified mesh and the number of points per interval (of the original mesh). Normally obtained via `build_basis_mesh(..)`.
    /// 
    fn build_derivative_basis(basis_mesh: (&Vec<Vertex1D>,f32)) -> Vec<Box<dyn Fn(f32) -> f32>>;

}

struct LinearBasis {
    unit_basis: [Box<dyn Fn(f32) -> f32>; 2]
}
struct CuadraticBasis {
    unit_basis: [Box<dyn Fn(f32) -> f32>; 3]
}
struct CubicBasis {
    unit_basis: [Box<dyn Fn(f32) -> f32>; 4]
}

impl FunctionBase1D<[Box<dyn Fn(f32) -> f32>; 2]> for LinearBasis {

    fn unit_basis(&self) -> &[Box<dyn Fn(f32) -> f32>; 2] {
        &self.unit_basis
    }

    fn build_basis_mesh(mesh: &Vec<Vertex1D>) -> (Vec<Vertex1D>,u32) {
        (mesh.clone(),2)
    }
    
    fn build_basis(basis_mesh: (&Vec<Vertex1D>,f32)) -> Vec<Box<dyn Fn(f32) -> f32>> {
        todo!()
    }

    fn build_equidistant_basis(start: f32, finish: f32, interval_number: u32) -> Vec<Box<dyn Fn(f32) -> f32>> {
        todo!()
    }

}
impl FunctionBase1D<[Box<dyn Fn(f32) -> f32>; 3]> for CuadraticBasis {

    fn unit_basis(&self) -> &[Box<dyn Fn(f32) -> f32>; 3] {
        &self.unit_basis
    }

    fn build_basis_mesh(mesh: &Vec<Vertex1D>) -> (Vec<Vertex1D>,u32) {
        todo!()
    }

    fn build_basis(basis_mesh: (&Vec<Vertex1D>,f32)) -> Vec<Box<dyn Fn(f32) -> f32>> {
        todo!()
    }

    fn build_equidistant_basis(start: f32, finish: f32, interval_number: u32) -> Vec<Box<dyn Fn(f32) -> f32>> {
        todo!()
    }

}
impl FunctionBase1D<[Box<dyn Fn(f32) -> f32>; 4]> for CubicBasis {

    fn unit_basis(&self) -> &[Box<dyn Fn(f32) -> f32>; 4] {
        &self.unit_basis
    }

    fn build_basis_mesh(mesh: &Vec<Vertex1D>) -> (Vec<Vertex1D>,u32) {
        todo!()
    }

    fn build_basis(basis_mesh: (&Vec<Vertex1D>,f32)) -> Vec<Box<dyn Fn(f32) -> f32>> {
        todo!()
    }

    fn build_equidistant_basis(start: f32, finish: f32, interval_number: u32) -> Vec<Box<dyn Fn(f32) -> f32>> {
        todo!()
    }

}