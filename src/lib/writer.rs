use crate::Error;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Instant;

/// # General Information
/// 
/// Writes solution of equation to a given file.
/// Can take a path to write to, names of variables and a prefix for all files.
/// Struct is meant to run on it's own thread to block as little as possible the execution of DzahuiWindow
/// 
/// # Fields
/// 
/// * `receiver` - A sync_channel receiver to obtain vector values to write to file 
/// * `write_path` - A directory to write files in
/// * `variable_names` - Chosen by a given equation. Normally a vector like ['x','y','z'] or similar
/// * `file_prefix` - To identify files from a single simulation
/// 
pub(crate) struct Writer {
    pub(crate) receiver: Receiver<Vec<f64>>,
    write_path: PathBuf,
    variable_names: Vec<&'static str>,
    file_prefix: String
}

impl Writer {

    /// # General Information
    /// 
    /// Creates a new instance of writer (And there should be only one, but it is not enforced).
    /// Can be told to erase previous files on a directory.
    /// Will complain if directory is not present
    /// 
    /// # Parameters
    /// 
    /// * `receiver` - A receiver to obtain the solution to an equation
    /// * `write_path` - The path in which files are created and written to
    /// * `file_prefix` - Prefix for all files of a given simulation
    /// * `variable_names` - A vector with all variables of a problem. Chosen by the equation struct in dzahui window. Also determines how many elements
    /// from solution vector are taken per line
    /// * `erase_prev_dir` - Option to erase every file inside dir given. Will not erase nested directories
    /// 
    pub(crate) fn new<A, B, C>(
        receiver: Receiver<Vec<f64>>,
        write_path: B,
        file_prefix: A,
        variable_names: C,
        erase_prev_dir: bool
    ) -> Result<Self,Error> where
        A: AsRef<str>,
        B: AsRef<str>,
        C: IntoIterator<Item = &'static str> {

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

    /// # General Information
    /// 
    /// Writes once to a file created inside. Will create a file for every call.
    /// To make every file unique, an id must be passed. Dzahui window will pass the time in milis, but any other unique f64 value will do.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - A reference to itself to use `write_path` and `file_prefix`
    /// * `id` - A unique id for a file
    /// * `vals` - a vector with values to write to file
    /// 
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

pub(crate) fn spawn(writer: Writer, timer: Instant) {
    thread::spawn(move || {
        loop {
            if let Ok(vals) = writer.receiver.recv() {
                
                let time = timer.elapsed().as_secs_f64();
                let res = writer.write(time, vals);
                // Send result back to main thread
                match res {
                    Ok(()) => log::info!("Data has been saved"),
                    Err(e) => panic!("Something happened between threads. Pleas report error to developer!: {}",e) 
                }
            
            } else {
                break;
            }
        }
    });
}