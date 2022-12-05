use crate::Error;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

pub(crate) struct Writer {
    pub(crate) receiver: Receiver<Vec<f64>>,
    write_path: PathBuf,
    variable_names: Vec<&'static str>,
    file_prefix: String
}

impl Writer {

    /// Creates a new instance of writer (And there should be only one, but it is not enforced). Can be told to erase previous files.
    pub(crate) fn new<A: AsRef<str>, B: AsRef<str>, C: IntoIterator<Item = &'static str>>(receiver: Receiver<Vec<f64>>, write_path: B,
        file_prefix: A, variable_names: C, erase_prev_dir: bool) -> Result<Self,Error> {

        let write_path = PathBuf::from(write_path.as_ref().to_string());

        if !write_path.as_path().exists() {
            return Err(Error::NotFound("Path for creating files and writing values not found"))
        }
        
        if erase_prev_dir {
            let files = fs::read_dir(write_path.clone())?;
            for file in files {
                
                let file = match file {
                    Ok(f) => f,
                    Err(e) => return Err(Error::Io(e))
                };

                if !file.file_type()?.is_dir() {
                    fs::remove_file(file.path())?;
                    log::info!("File: {:?} has been erased",file.path());
                }
            }
        }

        Ok(Self {
            receiver,
            write_path,
            variable_names: variable_names.into_iter().collect(),
            file_prefix: file_prefix.as_ref().to_string()
        })
    }

    /// Writes once to a file created inside
    pub(crate) fn write(&self, id: f64, vals: Vec<f64>) -> Result<(),Error> {

        //Create file name
        let mut file_name = self.file_prefix.clone();
        file_name.push_str(id.to_string().as_str());
        file_name.push_str(".csv");
        
        // Create file path
        let mut file_path = self.write_path.clone();
        file_path.push(file_name);

        // Create file
        let mut file = File::create(file_path)?;

        // Write varaibles
        let variables_len = self.variable_names.len();
        let mut header = self.variable_names.iter().fold(String::from(""), |mut prev, cur| {
            prev.push_str(cur);
            prev.push(',');
            prev
        });

        // Erase last comma
        header.pop();
        header.push('\n');

        file.write(header.as_bytes())?;

        // Write values
        for point in vals.chunks(variables_len) {
            let mut line = String::new();
            
            for e in point {
                line.push_str(e.to_string().as_str());
                line.push(',');
            }

            // Erase last comma
            line.pop();
            // Add line jump
            line.push('\n');

            file.write(line.as_bytes())?;

        }

        Ok(())

    }


}