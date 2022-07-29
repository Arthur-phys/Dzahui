#[derive(Debug)]
enum Error {
    File(std::io::Error),
    Custom(String),
    Unimplemented
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let content = match self {
            Error::File(e) => format!("{}", e),
            Error::Custom(e) => format!("{}", e),
            Error::Unimplemented => format!("este error no debería existir, favor de reportar con el desarrollador")
        };
        write!(formatter, "{}", content)
    }
}

impl std::error::Error for Error{}

impl Error {
    fn custom<A: Into<String>>(message: A) -> Self {
        Error::Custom(message.into())
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::File(source)
    }
}