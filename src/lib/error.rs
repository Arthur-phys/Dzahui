use std::{num::{ParseIntError, ParseFloatError}, ffi::NulError};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ImageError(image::ImageError),
    ParseInt(ParseIntError),
    CharacterError(String),
    NullCString(NulError),
    ParseFloat(ParseFloatError),
    Matrix(&'static str),
    FloatConversion,
    WrongDims,
    Infallible,
    Custom(String),
    ExtensionNotAllowed(String, String),
    Overflow,
    PieceWiseDims,
    Unimplemented,
    NotFound(&'static str),
    Parse(&'static str),
    MeshParse(String),
    Integration(String),
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
            Error::Integration(e) => format!("Error on integration method occurred: {}",e)
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