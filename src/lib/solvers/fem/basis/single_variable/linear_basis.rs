use crate::Error;

use super::polynomials_1d::FirstDegreePolynomial;
use super::piecewise_polynomials_1d::PiecewiseFirstDegreePolynomial;
use super::Composable1D;

/// # General Information
///
/// A Linear Basis is a set composed entirely of linearly independent functions that also generate the space in which they are contained.
/// In this case, functions are piecewise first-degree polynomials
///
/// # Fields
///
/// * `basis` - A vector of `PieceWiseFirstDegreePolynomial`.
///
pub(crate) struct LinearBasis {
    pub(crate) basis: Vec<PiecewiseFirstDegreePolynomial>,
}

impl LinearBasis {
    /// # General information
    ///
    /// Creation of a LinearBasis from a 1D mesh. may change to also provide 2D functionality.
    /// Obtains every function from the original two and a series of transformations.
    /// First and last functions are defined in four intervals even though they're needed only in three. This is done so that the vector `basis` contains always
    /// the same implementation of PieceWiseFirstDegreePolynomial.
    ///
    /// # Parameters
    ///
    /// * `self` - Consumes self to return a LinearBasis
    /// * `mesh` - A reference to the original mesh of points (may be filtered to omit RGB values).
    ///
    pub(crate) fn new(mesh: &Vec<f64>) -> Result<LinearBasis,Error> {

        // Left-side function
        let transformation = FirstDegreePolynomial::transformation_to_0_1(mesh[0], mesh[1]);
        let initial_transform_function = FirstDegreePolynomial::phi_2().compose(transformation);

        // First function is generated.
        let first_function = PiecewiseFirstDegreePolynomial::from_polynomials(
            vec![
                FirstDegreePolynomial::zero(),
                initial_transform_function,
                FirstDegreePolynomial::zero()
            ],
            vec![mesh[0], mesh[1]],
        )?;

        let mut basis_vec = vec![first_function];

        // Every other function is generated. Observe a double zip that generates triads of values needed to generate a single function in every interval.
        mesh.iter()
            .zip(mesh.iter().skip(1))
            .zip(mesh.iter().skip(2))
            .map(|((prev, cur), next)| -> Result<(),Error> {

                let transformation = FirstDegreePolynomial::transformation_to_0_1(*prev, *cur);
                let basis_left = FirstDegreePolynomial::phi_1().compose(transformation);
                let transformation = FirstDegreePolynomial::transformation_to_0_1(*cur, *next);
                let basis_right = FirstDegreePolynomial::phi_1().compose(transformation);

                let piecewise_function = PiecewiseFirstDegreePolynomial::from_polynomials(
                    vec![FirstDegreePolynomial::zero(), basis_left, basis_right, FirstDegreePolynomial::zero()],
                    vec![*prev, *cur, *next],
                )?;

                basis_vec.push(piecewise_function);

                Ok(())
            });

        // Last function is generated.
        let transformation =
        FirstDegreePolynomial::transformation_to_0_1(mesh[mesh.len() - 2], mesh[mesh.len() - 1]);
        let final_transform_function = FirstDegreePolynomial::phi_1().compose(transformation);
        
        let final_function = PiecewiseFirstDegreePolynomial::from_polynomials(
            vec![
                FirstDegreePolynomial::zero(),
                final_transform_function,
                FirstDegreePolynomial::zero(),
            ],
            vec![
                mesh[mesh.len() - 2],
                mesh[mesh.len() - 1],
            ],
        )?;

        basis_vec.push(final_function);

        Ok(
            LinearBasis { basis: basis_vec }
        )
    }
}

#[cfg(test)]
mod test {

    use super::PiecewiseFirstDegreePolynomial;
    use super::LinearBasis;

    #[test]
    fn transform_basis_three_nodes() {
        let mesh = vec![0_f64, 1_f64, 2_f64];
        let transformed = LinearBasis::new(&mesh).unwrap();

        assert!(transformed.basis.len() == 3);

        let first_pol = PiecewiseFirstDegreePolynomial::from_values(
            vec![0_f64, 0_f64, -1_f64, 0_f64],
            vec![0_f64, 0_f64, 1_f64, 0_f64],
            vec![-1_f64, 0_f64, 1_f64],
        ).unwrap();
        let second_pol = PiecewiseFirstDegreePolynomial::from_values(
            vec![0_f64, 1_f64, -1_f64, 0_f64],
            vec![0_f64, 0_f64, 2_f64, 0_f64],
            vec![0_f64, 1_f64, 2_f64],
        ).unwrap();
        let third_pol = PiecewiseFirstDegreePolynomial::from_values(
            vec![0_f64, 1_f64, 0_f64, 0_f64],
            vec![0_f64, -1_f64, 0_f64, 0_f64],
            vec![1_f64, 2_f64, 3_f64],
        ).unwrap();

        assert!(transformed.basis[0] == first_pol);
        assert!(transformed.basis[1] == second_pol);
        assert!(transformed.basis[2] == third_pol);
    }
}
