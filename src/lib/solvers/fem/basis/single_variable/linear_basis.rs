use super::polynomials_1d::{FirstDegreePolynomial, Composable1D};
use super::piecewise_polynomials_1d::{PiecewiseFirstDegreePolynomial};

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
    /// Returns a LinearBasisFactory, with only three functions representing the original [0,1] basis with cardinality 2 and a zero function.
    pub(crate) fn new() -> LinearBasisFactory {
        let phi_1 = FirstDegreePolynomial::new(1_f64, 0_f64);
        let phi_2 = FirstDegreePolynomial::new(-1_f64, 1_f64);
        let zero = FirstDegreePolynomial::zero();
        LinearBasisFactory { phi_1, phi_2, zero }
    }
}

/// # General Information
///
/// A LinearBasisFactory abstraacts completely the creation of a basis from the three original functions via transformations. May change in the future.
///
/// # Fields
///
/// * `phi_1` - identity function.
/// * `phi_2` - 1-x function.
/// * `zero` - 0 function.
///
pub(crate) struct LinearBasisFactory ();
impl LinearBasisFactory {
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
    pub(crate) fn with_mesh(self, mesh: &Vec<f64>) -> LinearBasis {

        // Left-side function
        let transformation = FirstDegreePolynomial::transformation_to_0_1(mesh[0], mesh[1]);
        let initial_transform_function = self.phi_2.compose(transformation);

        // First function is generated, note the extra point 'mesh[0]-1'
        let mut basis_vec = vec![PiecewiseFirstDegreePolynomial::from_polynomials(
            [
                &self.zero,
                &self.zero,
                &initial_transform_function,
                &self.zero,
            ],
            vec![mesh[0] - 1_f64, mesh[0], mesh[1]],
        )];

        // Every other function is generated. Observe a double zip that generates triads of values needed to generate a single function in every interval it is
        // not zero
        mesh.iter()
            .zip(mesh.iter().skip(1))
            .zip(mesh.iter().skip(2))
            .for_each(|((prev, cur), next)| {
                let transformation = FirstDegreePolynomial::transformation_to_0_1(*prev, *cur);
                let basis_left = self.phi_1.compose(transformation);
                let transformation = FirstDegreePolynomial::transformation_to_0_1(*cur, *next);
                let basis_right = self.phi_2.compose(transformation);

                basis_vec.push(PiecewiseFirstDegreePolynomial::from_polynomials(
                    [&self.zero, &basis_left, &basis_right, &self.zero],
                    vec![*prev, *cur, *next],
                ))
            });

        // Last function is generated. note the extra point 'mesh[mesh.len()-1] + 1.0'.
        let transformation =
        FirstDegreePolynomial::transformation_to_0_1(mesh[mesh.len() - 2], mesh[mesh.len() - 1]);
        let final_transform_function = self.phi_1.compose(transformation);

        basis_vec.push(PiecewiseFirstDegreePolynomial::from_polynomials(
            [
                &self.zero,
                &final_transform_function,
                &self.zero,
                &self.zero,
            ],
            vec![
                mesh[mesh.len() - 2],
                mesh[mesh.len() - 1],
                mesh[mesh.len() - 1] + 1_f64,
            ],
        ));

        LinearBasis { basis: basis_vec }
    }
}

#[cfg(test)]
mod test {

    use super::super::{FirstDegreePolynomial, PiecewiseFirstDegreePolynomial};
    use super::LinearBasis;

    #[test]
    fn create_unit_basis() {
        let unit_base = LinearBasis::<[f64; 3], [f64; 4]>::new();
        assert!(unit_base.phi_1 == FirstDegreePolynomial::new(1_f64, 0_f64));
        assert!(unit_base.phi_2 == FirstDegreePolynomial::new(-1_f64, 1_f64));
    }

    #[test]
    fn transform_basis_three_nodes() {
        let unit_base = LinearBasis::<[f64; 3], [f64; 4]>::new();
        let mesh = vec![0_f64, 1_f64, 2_f64];
        let transformed = unit_base.with_mesh(&mesh);

        assert!(transformed.basis.len() == 3);

        let first_pol = PiecewiseFirstDegreePolynomial::new(
            [0_f64, 0_f64, -1_f64, 0_f64],
            [0_f64, 0_f64, 1_f64, 0_f64],
            [-1_f64, 0_f64, 1_f64],
        );
        let second_pol = PiecewiseFirstDegreePolynomial::new(
            [0_f64, 1_f64, -1_f64, 0_f64],
            [0_f64, 0_f64, 2_f64, 0_f64],
            [0_f64, 1_f64, 2_f64],
        );
        let third_pol = PiecewiseFirstDegreePolynomial::new(
            [0_f64, 1_f64, 0_f64, 0_f64],
            [0_f64, -1_f64, 0_f64, 0_f64],
            [1_f64, 2_f64, 3_f64],
        );

        assert!(transformed.basis[0] == first_pol);
        assert!(transformed.basis[1] == second_pol);
        assert!(transformed.basis[2] == third_pol);
    }
}
