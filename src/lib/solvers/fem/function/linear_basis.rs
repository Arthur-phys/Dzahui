
use super::{PiecewiseFirstDegreePolynomial,FirstDegreePolynomial,TransformationFactory,IntervalStep,NumberOfArguments,Affine};
pub(crate) struct LinearBasis {
    pub(crate) basis: Vec<FirstDegreePolynomial>,
    pub(crate) transformation: TransformationFactory
}

pub(crate) struct PiecewiseLinearBasis<A: IntervalStep, B: NumberOfArguments> {
    pub(crate) basis: Vec<PiecewiseFirstDegreePolynomial<A,B>>,
    pub(crate) transformation: TransformationFactory
}

impl<A: IntervalStep, B: NumberOfArguments> PiecewiseLinearBasis<A, B> { 
    pub(crate) fn new(basis: Vec<PiecewiseFirstDegreePolynomial<A,B>>) -> Self {
        Self {
            basis,
            transformation: TransformationFactory()
        }
    }
}

impl LinearBasis {

    pub(crate) fn new_unit() -> Self {
        let phi_1 = FirstDegreePolynomial::new(1_f64,0_f64);
        let phi_2 = FirstDegreePolynomial::new(-1_f64,1_f64);
        Self {
            basis: vec![phi_1,phi_2],
            transformation: TransformationFactory()
        }
    }

    pub(crate) fn transform_basis(self, mesh: &Vec<f64>) -> PiecewiseLinearBasis<[f64;3],[f64;4]> {

        let phi_1 = &self.basis[0];
        let phi_2= &self.basis[1];

        let zero = FirstDegreePolynomial::zero();

        let transformation = self.transformation.build(mesh[0],mesh[1]);
        let initial_transform_function = phi_2.compose(transformation);

        let mut basis_vec = vec![
            PiecewiseFirstDegreePolynomial::from_polynomials([&zero,&zero,&initial_transform_function,&zero], [mesh[0]-1_f64,mesh[0],mesh[1]])
        ];

        mesh.iter().zip(mesh.iter().skip(1)).zip(mesh.iter().skip(2)).for_each(|((prev, cur), next)| {
            
            let transformation = self.transformation.build(*prev,*cur);
            let basis_left = phi_1.compose(transformation);
            let transformation = self.transformation.build(*cur, *next);
            let basis_right = phi_2.compose(transformation);
            
            basis_vec.push(
                PiecewiseFirstDegreePolynomial::from_polynomials([&zero,&basis_left,&basis_right,&zero], [*prev,*cur,*next])
            )
        });
        
        let transformation = self.transformation.build(mesh[mesh.len()-2],mesh[mesh.len()-1]);
        let final_transform_function = phi_1.compose(transformation);

        basis_vec.push(
            PiecewiseFirstDegreePolynomial::from_polynomials([&zero,&final_transform_function,&zero,&zero],
                 [mesh[mesh.len()-2],mesh[mesh.len()-1],mesh[mesh.len()-1] + 1_f64])
        );

        PiecewiseLinearBasis::new(basis_vec)
    }
}

#[cfg(test)]
mod test {

    use super::LinearBasis;
    use super::super::{FirstDegreePolynomial,PiecewiseFirstDegreePolynomial};
    

    #[test]
    fn create_unit_basis() {
        
        let unit_base = LinearBasis::new_unit();
        let basis = unit_base.basis;
        assert!(basis[0] == FirstDegreePolynomial::new(1_f64,0_f64));
        assert!(basis[1] == FirstDegreePolynomial::new(-1_f64,1_f64));
    }

    #[test]
    fn transform_basis_three_nodes() {

        let unit_base = LinearBasis::new_unit();
        let mesh =vec![0_f64,1_f64,2_f64];
        let transformed = unit_base.transform_basis(&mesh);

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