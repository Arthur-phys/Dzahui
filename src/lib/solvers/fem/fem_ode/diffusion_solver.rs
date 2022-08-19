// use super::{Vertex1D, BoundaryVertex1D, PolynomialDegree, DiffEquationSolver};

// struct DiffussionSolverBuilder {
//     boundary_vertices: [BoundaryVertex1D; 2],
//     vertices: Option<Vec<Vertex1D>>,
//     polynomial_degree: Option<PolynomialDegree>,
// }

// impl DiffussionSolverBuilder {
//     fn build(self) -> DiffussionSolver {
//         todo!()
//     }
// }

// struct DiffussionSolver {
//     boundary_vertices: [BoundaryVertex1D; 2],
//     vertices: Vec<Vertex1D>,
// }

// impl DiffEquationSolver for DiffussionSolver {
//     fn solve() -> Vec<f32> {
//         todo!()
//     }
// }

struct Function<'a> {
    function: Box<dyn Fn(f32) -> f32>,
    derivatives: Vec<Box<dyn Fn(f32) -> f32>>,
    composed: Vec<&'a Function<'a>>,
    c_class: usize
}

impl<'a> Function<'a> {
    fn new(function: Box<dyn Fn(f32) -> f32>, mut derivatives: Vec<Box<dyn Fn(f32) -> f32>>) -> Self {
        
        derivatives.reverse();
        let c_class = derivatives.len();

        Self {
            function,
            derivatives,
            c_class,
            composed: vec![]
        }
    }

    fn new_from_composition(function: Box<dyn Fn(f32) -> f32>, mut derivatives: Vec<Box<dyn Fn(f32) -> f32>>, refs: Vec<&'a Function>) -> Self {
        
        derivatives.reverse();
        let c_class = derivatives.len();
        
        Self {
            function,
            derivatives,
            c_class,
            composed: refs
        }
    }

    fn function(&self, x: f32) -> f32 {
        let f = &self.function;
        f(x)
    }
}

trait Differentiable {
    fn differentiate<'a>(&'a self, n: usize) -> Option<&'a Box<dyn Fn(f32) -> f32>>;
}


impl<'b> Differentiable for Function<'b> {
    fn differentiate<'a>(&'a self, n: usize) -> Option<&'a Box<dyn Fn(f32) -> f32>> {

        self.derivatives.get(n-1)

    }
}

struct LinearBasis<'a> {
    unit_basis: [Function<'a>; 2],
}

impl<'a> LinearBasis<'a> {

    fn transformation(start: f32, finish: f32) -> Function<'a> {
        Function::new(
            Box::new(move |x: f32| {
                        (x - start) / (finish - start)
                    }),
            vec![
                Box::new(move |x: f32| {
                    start / (finish - start)
                })
            ]
        )
    }

    fn create_basis(self, mesh: Vec<f32>) -> Vec<Function<'a>> {
        
        let phi_1 = &self.unit_basis[0];
        let phi_2: &'a Function = &self.unit_basis[1];

        let mut basis_vector: Vec<Function> = Vec::with_capacity(mesh.len());

        mesh.iter().zip(mesh.iter().skip(1)).enumerate().for_each(|(pos,(start,finish))| {

            let Function {function: transformation, derivatives, ..} = LinearBasis::transformation(*start,*finish);

            let phi_2_ref = phi_2.clone();
            // if pos == 0 {
                let interval_func = Function::new_from_composition( Box::new(move |x: f32| {
                    phi_2_ref.function(transformation(x))
                }),vec![], vec![phi_2_ref]);

                basis_vector.push(interval_func)

            // } else { }
                
            //     let interval_func = Function::new(Box::new(|x: f32| {
            //         if x > *start && x < *finish {
                    
            //             phi_2.function(transformation.function(x))
                    
            //         } else if x < *start && x > *&mesh[pos-1] {

            //             phi_1.function(transformation.function(x))
                    
            //         } else {

            //             0_f32
            //         }
            //     }),
            //     vec![Box::new(|x: f32| {
            //         if x > *start && x < *finish {
                    
            //             phi_2.function(transformation.function(x))
                    
            //         } else if x < *start && x > mesh[pos-1] {

            //             phi_1.function(transformation.function(x))
                    
            //         } else {

            //             0_f32
            //         }
            //     })]);
            
            // }
        });

        basis_vector

    }

}