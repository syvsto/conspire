use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum DimensionError {
    NoX,
    NoY,
}

impl fmt::Display for DimensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoX => write!(f, "Missing X dimension"),
            NoY => write!(f, "Missing X dimension"),
        }
    }
}

impl Error for DimensionError {
    fn description(&self) -> &str {
        "Couldn't parse X/Y dimension"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
    
