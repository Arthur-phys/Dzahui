// Module declaration
pub(crate) mod mesh_builder;
pub(crate) mod vertex_type;

// External dependencies
use cgmath::Matrix4;
use ndarray::Array1;
use num::ToPrimitive;

// Internal dependencies
use crate::{
    simulation::drawable::binder::{Binder, Bindable, Drawable},
    Error,
};
use mesh_builder::MeshBuilder;
use vertex_type::VertexType;

/// # General Information
///
/// Representation of a plane figure/ 3d body. Contains information to draw to screen and move/rotate mesh representation to final position.
///
/// # Fields
///
/// ## Numerical Integration Fields
///
/// * `conditions` - Kind a given vertex has. Can be a boundary or internal vertex. Also contains the possible initial/boundary condition.
///
/// ## Drawing Fields
///
/// * `max_length` - Maximum length of figure. Used to center camera arround objective.
/// * `model_matrix` - Translates and rotates object to final world position.
/// * `binder` - vao, vbo and ebo variables bound to mesh drawable in GPU.
/// * `indices` - Indices that map to vertices. Normally used in triads. Specified in gl configuration.
///
/// ## Shared Fields
///
/// * `vertices` -  Vertices in 3d space. Normally used in sextuples (coordinate and color). Specified in gl configuration.
///
#[derive(Debug)]
pub(crate) struct Mesh {
    pub(crate) conditions: Array1<VertexType>,
    pub(crate) max_length: f64,
    pub(crate) model_matrix: Matrix4<f32>,
    binder: Binder,
    pub(crate) indices: Array1<u32>,
    pub(crate) vertices: Array1<f64>,
}

impl Mesh {
    /// Getter for model_matrix
    pub fn get_model_matrix(&self) -> &Matrix4<f32> {
        &self.model_matrix
    }

    /// Creates new instance of builder
    pub fn builder<B>(location: B) -> MeshBuilder
    where
        B: AsRef<str>,
    {
        MeshBuilder::new(location)
    }

    // Filtering vertices to give to 1d solver. Temporal function. To be changed for better solution.
    pub(crate) fn filter_for_solving_1d(&self) -> Array1<f64> {
        // size of vertex is 6. There are double the vertices in 1d since a new pair is generated to draw a bar, therefore len is divided by 12.
        let vertices_len = self.vertices.len() / 12;
        self.vertices
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| {
                if idx % 6 == 0 && idx < vertices_len * 6 {
                    Some(*x)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Temporary solution to move gradient updating out of dzahui window. Probably will be changed in the future.
    /// Obtains max and min of solution (normallly some sort of rate of change), divides every element by the difference and then multiplies them by
    /// pi/2 so that, when calculating their sine and cosine, there's a mapping between max velocity <-> red and min velocity <-> blue
    pub(crate) fn update_gradient_1d(&mut self, velocity_norm: Vec<f64>) {
        let sol_max = velocity_norm
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);

        let sol_min = velocity_norm.iter().copied().fold(f64::INFINITY, f64::min);
        let vertices_len = self.vertices.len();
        
        for i in 0..(vertices_len / 12) {
            let norm_sol =
                (velocity_norm[i] - sol_min) / (sol_max - sol_min) * (std::f64::consts::PI / 2.);
            self.vertices[6 * i + 3] = norm_sol.sin();
            self.vertices[6 * i + 5] = norm_sol.cos();
            self.vertices[6 * i + 3 + vertices_len / 2] = norm_sol.sin();
            self.vertices[6 * i + 5 + vertices_len / 2] = norm_sol.cos();
        }
    }
}

impl Bindable for Mesh {
    fn get_binder(&self) -> Result<&Binder, Error> {
        Ok(&self.binder)
    }

    fn get_mut_binder(&mut self) -> Result<&mut Binder, Error> {
        Ok(&mut self.binder)
    }
}

impl Drawable for Mesh {
    fn get_triangles(&self) -> Result<&Array1<u32>, Error> {
        Ok(&self.indices)
    }

    fn get_vertices(&self) -> Result<Array1<f32>, Error> {
        Ok(Array1::from_iter(
            self.vertices.iter().map(|x| x.to_f32().unwrap()),
        ))
    }

    fn get_max_length(&self) -> Result<f32, Error> {
        let max_len = self.max_length.to_f32();

        match max_len {
            Some(f) => {
                if f.is_finite() {
                    Ok(f)
                } else {
                    Err(Error::Overflow)
                }
            }
            None => Err(Error::Unimplemented),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Mesh;
    use ndarray::Array1;

    #[test]
    fn parse_coordinates() {
        let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj")
            .build_mesh_3d()
            .unwrap();
        assert!(
            new_mesh.vertices
                == Array1::from_vec(vec![
                    -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 1.0
                ])
        );
        assert!(new_mesh.indices == Array1::from_vec(vec![0, 1, 2]));
    }

    #[test]
    fn is_max_distance() {
        let new_mesh = Mesh::builder("/home/Arthur/Tesis/Dzahui/assets/test.obj")
            .build_mesh_2d()
            .unwrap();
        assert!(new_mesh.max_length >= 1.90);
        assert!(new_mesh.max_length <= 2.10);
    }
}
