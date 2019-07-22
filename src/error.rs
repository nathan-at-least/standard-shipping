use clap;
use std;

#[derive(Debug)]
pub enum Error {
    ClapError(clap::Error),
    IOError(std::io::Error),
}

impl From<clap::Error> for Error {
    fn from(e: clap::Error) -> Error {
        Error::ClapError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IOError(e)
    }
}
