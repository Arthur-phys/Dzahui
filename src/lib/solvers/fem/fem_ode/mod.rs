pub mod diffusion_solver;
mod quadrature;
mod linear_solver;

use super::Vertex;

trait DiffEquationSolver {
    fn solve() -> Vec<f32>;
}

trait TimeDiffEquationSolver: DiffEquationSolver {
    fn do_step() -> Vec<f32>;
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

trait BasisLenght {}
impl BasisLenght for [Box<dyn Fn(f32) -> f32>; 2] {}
impl BasisLenght for [Box<dyn Fn(f32) -> f32>; 3] {}
impl BasisLenght for [Box<dyn Fn(f32) -> f32>; 4] {}

trait FunctionBase1D<A: BasisLenght> {
    
    fn interval_transformation(start: f32, finish: f32) -> Box<dyn Fn(f32) -> f32> {
        Box::new(move |x: f32| {
            (x - start) / (finish - start)
        })
    }
    fn unit_basis(&self) -> &A;
    fn build_basis_mesh(mesh: &Vec<Vertex1D>) -> (Vec<Vertex1D>,u32);
    fn build_basis(basis_mesh: (&Vec<Vertex1D>,f32)) -> Vec<Box<dyn Fn(f32) -> f32>>;
    fn build_equidistant_basis(start: f32, finish: f32) -> Vec<Box<dyn Fn(f32) -> f32>>;

}

trait DifferentiableBasis<A: BasisLenght>: FunctionBase1D<A> {

    fn interval_transformation_derivative(start: f32, finish: f32) -> f32 {
        1.0 / (finish - start)
    }
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

    fn build_equidistant_basis(start: f32, finish: f32) -> Vec<Box<dyn Fn(f32) -> f32>> {
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

    fn build_equidistant_basis(start: f32, finish: f32) -> Vec<Box<dyn Fn(f32) -> f32>> {
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

    fn build_equidistant_basis(start: f32, finish: f32) -> Vec<Box<dyn Fn(f32) -> f32>> {
        todo!()
    }

}