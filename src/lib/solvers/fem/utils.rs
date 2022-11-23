// Internal dependencies
use crate::Error;

// External dependencies
use ndarray::{Array1, Array2, Axis};


pub fn add(b: &Array1<f64>, v: &Array1<f64>) -> Result<Array1<f64>,Error> {

    if b.len() != v.len() {
        return Err(Error::WrongDims);
    }

    let len = b.len();
    let mut result_vec = Array1::from_vec(vec![0_f64;len]);

    for i in 0..=(len-1) {
        result_vec[i] = b[i] + v[i];
    }

    Ok(result_vec)
}

pub fn tridiagonal_matrix_vector_multiplication(a: &Array2<f64>, b: &Array1<f64>, c: f64) -> Result<Array1<f64>,Error> {
    
    if !a.is_square() || b.len() != a.len_of(Axis(0)) {
        return Err(Error::WrongDims);
    }
    
    // get number of operations to perform
    let len = b.len();
    // initialize result vector
    let mut result_vec = Array1::from_elem(len,0_f64);

    for i in 1..=(len - 2) {
        
        result_vec[i] = c * (a[[i,i-1]] * b[i-1] + a[[i,i]]* b[i] + a[[i,i+1]] * b[i+1])

    }

    result_vec[0] = c * ( a[[0,0]]* b[0] + a[[0,1]] * b[1] );
    result_vec[len-1] = c * ( a[[len-1,len-2]]* b[len-2] + a[[len-1,len-1]] * b[len-1] );

    Ok(result_vec)
}