#[derive(Debug)]
#[allow(dead_code)]
enum ErrorKind {
    Io(std::io::Error),
    WrongDims,
    Custom(String),
    Unimplemented
}

#[derive(Debug)]
#[allow(dead_code)]
struct RealError {
    internal: ErrorKind,
    helper_message: Option<String>    
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    WrongDims,
    Custom(String),
    Unimplemented
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let content = match self {
            Error::Io(e) => format!("io error, {}", e),
            Error::WrongDims => format!("one or more of the provided elements do not have the correct dimensions"),
            Error::Custom(e) => format!("{}", e),
            Error::Unimplemented => format!("este error no debería existir, favor de reportar con el desarrollador")
        };
        write!(formatter, "{}", content)
    }
}

impl std::error::Error for Error{}

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