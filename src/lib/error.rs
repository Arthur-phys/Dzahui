#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    WrongDims,
    Custom(String),
    ExtensionNotAllowed(String, String),
    Overflow,
    PieceWiseDims,
    Unimplemented,
    Parse(String),
    Integration(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let content = match self {
            Error::Io(e) => format!("io error, {}", e),
            Error::WrongDims => {
                format!("one or more of the provided elements do not have the correct dimensions")
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
