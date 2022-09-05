
use super::{PiecewiseFirstDegreePolynomial,FirstDegreePolynomial,TransformationFactory,IntervalStep,NumberOfArguments,Affine};

pub(crate) struct LinearBasis<A: IntervalStep, B: NumberOfArguments> {
    pub(crate) basis: Vec<PiecewiseFirstDegreePolynomial<A,B>>,
}

impl<A: IntervalStep, B: NumberOfArguments> LinearBasis<A, B> { 
    
    pub(crate) fn new() -> LinearBasisFactory {
        
        let phi_1 = FirstDegreePolynomial::new(1_f64,0_f64);
        let phi_2 = FirstDegreePolynomial::new(-1_f64,1_f64);
        let zero = FirstDegreePolynomial::zero();
        LinearBasisFactory {
            phi_1,
            phi_2,
            zero,
        }
    }
}

pub(crate) struct LinearBasisFactory {
    phi_1: FirstDegreePolynomial,
    phi_2: FirstDegreePolynomial,
    zero: FirstDegreePolynomial,
}

impl LinearBasisFactory {

    pub(crate) fn with_equidistant_mesh(self, beg: f64, end: f64, n: f64) -> LinearBasis<[f64;3],[f64;4]> {
        todo!()
    }

    pub(crate) fn with_mesh(self, mesh: &Vec<f64>) -> LinearBasis<[f64;3],[f64;4]> {

        let transformation_factory = TransformationFactory {};

        let transformation = transformation_factory.build(mesh[0],mesh[1]);
        let initial_transform_function = self.phi_2.compose(transformation);

        let mut basis_vec = vec![
            PiecewiseFirstDegreePolynomial::from_polynomials([&self.zero,&self.zero,&initial_transform_function,&self.zero], [mesh[0]-1_f64,mesh[0],mesh[1]])
        ];

        mesh.iter().zip(mesh.iter().skip(1)).zip(mesh.iter().skip(2)).for_each(|((prev, cur), next)| {
            
            let transformation = transformation_factory.build(*prev,*cur);
            let basis_left = self.phi_1.compose(transformation);
            let transformation = transformation_factory.build(*cur, *next);
            let basis_right = self.phi_2.compose(transformation);
            
            basis_vec.push(
                PiecewiseFirstDegreePolynomial::from_polynomials([&self.zero,&basis_left,&basis_right,&self.zero], [*prev,*cur,*next])
            )
        });
        
        let transformation = transformation_factory.build(mesh[mesh.len()-2],mesh[mesh.len()-1]);
        let final_transform_function = self.phi_1.compose(transformation);

        basis_vec.push(
            PiecewiseFirstDegreePolynomial::from_polynomials([&self.zero,&final_transform_function,&self.zero,&self.zero],
                 [mesh[mesh.len()-2],mesh[mesh.len()-1],mesh[mesh.len()-1] + 1_f64])
        );

        LinearBasis {basis: basis_vec}
    }
}

#[cfg(test)]
mod test {

    use super::LinearBasis;
    use super::super::{FirstDegreePolynomial,PiecewiseFirstDegreePolynomial};
    

    #[test]
    fn create_unit_basis() {
        
        let unit_base = LinearBasis::<[f64;3],[f64;4]>::new();
        assert!(unit_base.phi_1 == FirstDegreePolynomial::new(1_f64,0_f64));
        assert!(unit_base.phi_2 == FirstDegreePolynomial::new(-1_f64,1_f64));
    }

    #[test]
    fn transform_basis_three_nodes() {

        let unit_base = LinearBasis::<[f64;3],[f64;4]>::new();
        let mesh =vec![0_f64,1_f64,2_f64];
        let transformed = unit_base.with_mesh(&mesh);

        assert!(transformed.basis.len() == 3);

        let first_pol = PiecewiseFirstDegreePolynomial::new([0_f64,0_f64,-1_f64,0_f64], 
            [0_f64,0_f64,1_f64,0_f64], [-1_f64,0_f64,1_f64]);
        let second_pol = PiecewiseFirstDegreePolynomial::new([0_f64,1_f64,-1_f64,0_f64], 
            [0_f64,0_f64,2_f64,0_f64], [0_f64,1_f64,2_f64]);
        let third_pol = PiecewiseFirstDegreePolynomial::new([0_f64,1_f64,0_f64,0_f64], 
            [0_f64,-1_f64,0_f64,0_f64], [1_f64,2_f64,3_f64]);

        assert!(transformed.basis[0] == first_pol);
        assert!(transformed.basis[1] == second_pol);
        assert!(transformed.basis[2] == third_pol);
    }
}