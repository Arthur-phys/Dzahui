use std::{num::{ParseIntError, ParseFloatError}, ffi::NulError, sync::mpsc::RecvError};

#[derive(Debug)]
/// # General Information
/// 
/// Error enum to use on all functions. Contains personalized errors with a corresponding explanation
/// 
/// # Arms
/// 
/// * `ExtensionNotAllowed` - Files like shaders, meshes or font maps need a specific extension. When the extension is not appropiate, this error is thrown
/// * `ImageError` - Errors related to image reading and parsing
/// * `ParseFloat` - Error while parsing a float
/// * `ParseInt` - Error while parsing an int
/// * `CharacterError` - Error while creating the character set for the window
/// * `NotFound` - Error while looking for files
/// * `NullCString` - Error while converting to c-types
/// * `Matrix` - Errors ocurring while using matrices
/// * `Parse` - Error while interpreting files
/// * `Integration` - Error on numeric integration
/// * `Io` - Error on IO operations
/// * `MeshParse` - Error while parsing a mesh
/// * `FloatConversion` - Error on float conversion betweeen f64 and f32
/// * `Custom` - Less common errors
/// * `PieceWiseDims` - Error while creating a piecewise function
/// * `Unimplemented` - Error that should not exist
/// * `Infallible` - Error that never happens
/// * `WrongDims` - Error while operating on vectors and matrices
/// * `Overflow` - Error when a number overflows
/// * `Receiver` - Error on communication between threads
/// * `Writing` - Error while writing to file values of equation
/// 
pub enum Error {
    ExtensionNotAllowed(String, String),
    ImageError(image::ImageError),
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    CharacterError(String),
    BoundaryError(String),
    NotFound(&'static str),
    NullCString(NulError),
    Matrix(&'static str),
    Parse(&'static str),
    Integration(String),
    Io(std::io::Error),
    MeshParse(String),
    FloatConversion,
    Custom(String),
    PieceWiseDims,
    Unimplemented,
    Infallible,
    WrongDims,
    Overflow,
    Receiver(RecvError),
    Writing,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let content = match self {
            Error::CharacterError(s) => format!("Error while using character set: {}",s),
            Error::NullCString(e) => format!("Error while interacting with OpenGL: {}",e),
            Error::FloatConversion => format!("Unable to convert between f32 and f64"),
            Error::Infallible => format!("This error can not happen"),
            Error::Matrix(s) => format!("Matrix operation failed {}",s),
            Error::MeshParse(s) => format!("Unable to parse mesh file: {}",s),
            Error::ParseFloat(e) => format!("ParseFloat error: {}",e),
            Error::ParseInt(e) => format!("ParseInt error: {}",e),
            Error::NotFound(file) => format!("Could not find file: {}",file),
            Error::BoundaryError(e) => format!("Boundary error: {}",e),
            Error::Io(e) => format!("IO error: {}", e),
            Error::ImageError(e) => format!("Image error: {}",e),
            Error::WrongDims => {
                format!("One or more of the provided elements do not have the correct dimensions")
            }
            Error::Custom(e) => format!("{}", e),
            Error::ExtensionNotAllowed(file, action) => {
                format!("Extension of file {} is not allowed for {}", file, action)
            }
            Error::Overflow => String::from("Overflow occurred"),
            Error::Parse(e) => format!("Error while parsing file: {}", e),
            Error::PieceWiseDims => format!("Number of arguments must be one more than number of breakpoints for a piecewise function definition to make sense"),
            Error::Unimplemented => {
                format!("This error should not exist. Report it to the developer")
            },
            Error::Integration(e) => format!("Error on integration method occurred: {}",e),
            Error::Writing => format!("Error while writing to file values of differential equation"),
            Error::Receiver(e) => format!("No message received on thread: {}",e)
        };
        write!(formatter, "{}", content)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn custom<A: Into<String>>(message: A) -> Self {
        Error::Custom(message.into())
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::Io(source)
    }
}

impl From<image::ImageError> for Error {
    fn from(source: image::ImageError) -> Self {
        Error::ImageError(source)
    }
}

impl From<ParseIntError> for Error {
    fn from(source: ParseIntError) -> Self {
        Error::ParseInt(source)
    }
}

impl From<ParseFloatError> for Error {
    fn from(source: ParseFloatError) -> Self {
        Error::ParseFloat(source)
    }
}

impl From<NulError> for Error {
    fn from(source: NulError) -> Self {
        Error::NullCString(source)
    }
}

impl From<RecvError> for Error {
    fn from(source: RecvError) -> Self {
        Error::Receiver(source)
    }
}