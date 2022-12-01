// External dependencies
extern crate log;
extern crate chrono;
extern crate regex;

use regex::Regex;
use log::{Log, Record, Level, Metadata};
use std::fs::{File, OpenOptions, read_dir};
use std::path::PathBuf;
use std::io::{prelude::*, BufReader};

// Communication with file writer
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;

// Print time adequately
use chrono::prelude::*;

/// # General Information
/// 
/// Struct that writes logs to files. Changes files when maximum number of lines has been reached.
/// 
/// # Fields
/// 
/// * `log_path` - Direction in which files will be generated
/// * `line_count` - Number of lines currently written 
/// * `line_maximum` - Maximum number of lines per file
/// * `current_log_file_number` - Number of current file in which logs are being written
/// * `log_file` - Current log file
/// * `rx` - Used for internal communication between log, warn and error in this structure
/// 
pub struct LogWriter {
    pub log_path: PathBuf,
    pub line_count: u64,
    pub line_maximum: u64,
    pub current_log_file_number: i64,
    pub log_file: File,
    pub rx: Receiver<String>
}

impl LogWriter {
    /// # General Information
    /// 
    /// Creates new instance of LogWriter while taking into account previous log files, last log file and state of last log file
    /// 
    /// # Parameters
    /// 
    /// * `log_path` - A path-like object where all logs remain
    /// * `rx` - A receiver for internal communication
    /// * `line_maximum` - Maximum number of lines for log file
    /// 
    pub fn new(log_path: PathBuf, rx: Receiver<String>, line_maximum: u64) -> LogWriter {
        if !log_path.as_path().exists() {
            panic!("Log folder does not exist!");
        }

        let reg = Regex::new(r"(?x)^trace-(?P<number>[0-9]+)\.log$").unwrap();
        // We look for the current log file number.
        let dir_iter = match read_dir(&log_path) {
            Ok(v) => v,
            Err(_) => panic!("Could not iterate through files in log dir")
        };
        let mut current_log_file_number: i64 = -1;
        for dir in dir_iter {
            match dir {
                Ok(v) => {
                    let filename = String::from(v.file_name().to_string_lossy());
                    match reg.captures(&filename).and_then(|cap| {
                        cap.name("number").map(|number| number.as_str().parse::<i64>().unwrap())
                    }) {
                        Some(v) => {
                            if v > current_log_file_number {
                                current_log_file_number = v;
                            }
                        },
                        None => ()
                    };
                },
                Err(_) => panic!("Could not see file inside log dir")
            };
        }
        // We will check the number of lines of the current log file.
        let (line_count, log_file_path) = if current_log_file_number == -1 {
            // No log yet
            current_log_file_number += 1;
            let mut file_path = log_path.clone();
            file_path.push(format!("trace-{}.log", current_log_file_number));
            (0, file_path.clone())
        } else {
            let mut log_file_path = log_path.clone();
            log_file_path.push(format!("trace-{}.log", current_log_file_number));
            match File::open(&log_file_path) {
                Ok(f) => {
                    let mut curr_line_count = 0;
                    for _line in BufReader::new(f).lines() {
                        curr_line_count += 1;
                    }
                    // Finally, we check how many lines it has
                    if curr_line_count < line_maximum {
                        (curr_line_count, log_file_path.clone())
                    } else {
                        // Time to create a new file
                        current_log_file_number += 1;
                        let mut file_path = log_path.clone();
                        file_path.push(format!("trace-{}.log", current_log_file_number));
                        (0, file_path.clone())
                    }
                },
                Err(e) => {
                    panic!("Could not count file lines: {}", e);
                }
            }
        };
        
        // With the chosen log path, we continue the log
        let f = if log_file_path.as_path().exists() {
            match OpenOptions::new()
                .write(true)
                .append(true)
                .open(&log_file_path) {
                Ok(v) => v,
                Err(_) => panic!("Imposible sobreescribir la bitácora.")
            }
        } else {
            match File::create(&log_file_path) {
                Ok(v) => v,
                Err(e) => panic!("Could not create log file {}! ({})", log_file_path.as_os_str().to_string_lossy(), e)
            }
        };
        LogWriter{
            log_path,
            log_file: f,
            rx: rx,
            line_count,
            line_maximum,
            current_log_file_number
        }
    }

    /// # General Information
    /// 
    /// Writes to log file and changes internal values like line number and log file if necessary
    /// 
    /// # Parameters
    /// 
    /// * `&mut self` - A mutable reference to write and change internal state
    /// 
    pub fn run(&mut self){
        for record in &self.rx {
            match self.log_file.write((record+"\n").as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Could not write to log file: {}", e)
                }
            }
            self.line_count = (self.line_count + 1) % self.line_maximum;
            if self.line_count == 0 {
                // Time to swap logs
                match self.log_file.flush() {
                    Ok(_) => (),
                    Err(_) => panic!("Could not flush contents")
                };
                self.current_log_file_number += 1;
                let mut log_file_path = self.log_path.clone();
                log_file_path.push(format!("trace-{}.log", self.current_log_file_number));
                self.log_file = match File::create(&log_file_path) {
                    //Ok(v) => BufWriter::with_capacity(2000, v),
                    Ok(v) => v,
                    Err(e) => panic!("Could not create log file {}! ({})", log_file_path.as_os_str().to_string_lossy(), e)
                }
            }
        }
    }
}

/// # General Information
/// 
/// Logger structure for Dzahui
/// 
/// # Fields
/// 
/// * `print_to_term` - Wether log should be printed to standard exit or not
/// * `print_to_file` - Wether log should be written to file
/// * `tx` - Communication between thread from logs and the rest of Dzahui
/// * `logger_id` - String to print on log
/// 
pub struct DzahuiLogger {
    print_to_term: bool,
    print_to_file: bool,
    tx: SyncSender<String>,
    logger_id: &'static str
}

impl Log for DzahuiLogger {
    /// # General Information
    /// 
    /// Indicates which level of log will be used
    /// 
    /// # Parameters
    /// 
    /// * `&self` - to acess method via '.'
    /// * `metadata` - to check metadata level
    /// 
    fn enabled(&self, metadata: &Metadata) -> bool {
        match metadata.level() {
            Level::Error => true,
            Level::Warn => true,
            Level::Info => true,
            Level::Debug => true,
            Level::Trace => true
        }
    }

    /// # General Information
    /// 
    /// Deals with every log on an individual manner
    /// 
    /// # Parameters
    /// 
    /// * `&self` - An instance to check some internal state variables
    /// * `record` - Payload of a log message to record
    /// 
    fn log(&self, record: &Record) {
        // Only process messages we're interested on
        if self.enabled(record.metadata()) {
            let level_string = {
                match record.level() {
                    Level::Error => format!("\u{001b}[0;31m{}\u{001b}[0m", record.level().to_string()),
                    Level::Warn => format!("\u{001b}[0;33m{}\u{001b}[0m", record.level().to_string()),
                    Level::Info => format!("\u{001b}[0;36m{}\u{001b}[0m", record.level().to_string()),
                    Level::Debug => format!("\u{001b}[0;35m{}\u{001b}[0m", record.level().to_string()),
                    Level::Trace => format!("{}", record.level().to_string()),
                }
            };
            let registry = if cfg!(feature = "log-module") {format!(
                "{} {}[{:<5}, {}]: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                self.logger_id,
                level_string,
                record.module_path().unwrap_or_default(),
                record.args()
            )} else {format!(
                "{} {}[{:>16}]: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                self.logger_id,
                level_string,
                record.args()
            )}; 

            if self.print_to_term {
                println!("{}",&registry);
            }
            if self.print_to_file {
                match self.tx.send(registry) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Cannot write anymore to log file (thread crashed)");
                        panic!("Could not write anymore to the log file");
                    }
                };
            }
        }
    }

    /// Empty function
    fn flush(&self) {}
}

impl DzahuiLogger {
    /// # General Information
    /// 
    /// Creates a new instance of the logger
    /// 
    /// # Parameters
    /// 
    /// * `logger_id` - An id for this instance 
    /// * `print_to_term` - Wether to print to terminal or not 
    /// * `log_path` - Where to store logs
    ///  
    pub fn new(logger_id: &'static str, print_to_term: bool, log_path: Option<PathBuf>) -> DzahuiLogger {
        if let Some(log_path) = log_path {
            if !log_path.as_path().exists() {
                panic!("Could not find log path ({})", log_path.as_os_str().to_string_lossy());
            }
            // We generate the communication channel
            let (sender, receiver) = sync_channel::<String>(0);
            // This thread will receive all log messages
            thread::spawn(move || {
                LogWriter::new(log_path, receiver, 10_000_000).run()
            });
            DzahuiLogger{print_to_term, print_to_file: true, tx: sender, logger_id}
        } else {
            let (sender, _) = sync_channel::<String>(0);
            // The sender will anyways never be used
            DzahuiLogger{print_to_term, print_to_file: false, tx: sender, logger_id}
        }
    }
}

/// # General Information
/// 
/// Spawns a boxed logger
/// Must only be called once.
/// 
/// # Parameters
/// 
/// * `log_level` - Which level of logging to use 
/// * `prefix` - Id of logger
/// 
pub fn spawn(log_level: log::LevelFilter, prefix: &'static str) -> Result<(), log::SetLoggerError> {
    log::set_boxed_logger(Box::new(DzahuiLogger::new(prefix, true, None))).map(|()| 
        log::set_max_level(log_level)
    )
}