use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver};

use crate::Error;
use crate::mesh::mesh_builder::MeshDimension;
use crate::solvers::Solver;


pub(crate) struct Writer {
    receiver: Receiver<Vec<f64>>,
    variables: Vec<&'static str>,
    write_path: PathBuf,
    file_prefix: String
}

impl Writer {

    /// Creates a new instance of writer (And there should be only one but it is not enforced)
    fn new<A: AsRef<str>, B: AsRef<str>>(receiver: Receiver<Vec<f64>>, write_path: B,
        file_prefix: A, dimension: &MeshDimension) -> Result<Self,Error> {

        let write_path = PathBuf::from(write_path.as_ref().to_string());

        let variables: Vec<&'static str> = match dimension {
            MeshDimension::One => vec!["x"],
            MeshDimension::Two => vec!["x","y"],
            MeshDimension::Three => vec!["x","y","z"]
        };

        if !write_path.as_path().exists() {
            return Err(Error::NotFound("Path for creating files and writing values not found"))
        }

        Ok(Self {
            receiver,
            write_path,
            variables,
            file_prefix: file_prefix.as_ref().to_string()
        })
    }

    /// Writes once to a file created inside
    fn write(&self, id: usize) -> Result<(),Error> {
        
        // Create file path
        let mut file_path = self.write_path.clone();
        file_path.extend([self.file_prefix.clone(),id.to_string()]);

        // Create file
        let mut file = File::create(file_path)?;

        // Obtain values from receiver
        let val = self.receiver.recv()?;

        file.write("ava".as_bytes());
        

        todo!()

    }


}