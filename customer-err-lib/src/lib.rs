use std::fmt::write;


// use std::fmt;


#[derive(Debug)]
pub struct SuperError {
    side: SuperErrorSideKick,
}

impl std::fmt::Display for SuperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for SuperError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.side)
    }
}

#[derive(Debug)]
pub struct SuperErrorSideKick;

impl std::fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl std::error::Error for SuperErrorSideKick {}

pub fn get_super_error() -> core::result::Result<(), SuperError> {
    Err(SuperError { side: SuperErrorSideKick })
}




// -------------------------------------------------
// use std::io;
// use std::num;
// use std::str;

#[derive(Debug)]
pub enum CliError {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error)
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {

        match &self {
            CliError::ParseIntError(ref e) => Some(e),
            CliError::Utf8Error(ref e) => Some(e),
            CliError::IoError(ref e) => Some(e)
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match &self {
            CliError::ParseIntError(ref e) => e.fmt(f),
            CliError::Utf8Error(ref e) => e.fmt(f),
            CliError::IoError(ref e) => e.fmt(f)
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        println!("CliError--Error--from");
        CliError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for CliError {
    fn from(error: std::num::ParseIntError) -> Self {
        println!("CliError--ParseIntError--from");
        CliError::ParseIntError(error)
    }
}

impl From<std::str::Utf8Error> for CliError {
    fn from(error: std::str::Utf8Error) -> Self {
        println!("CliError--Utf8Error--from");
        CliError::Utf8Error(error)
    }
}


fn to_u32(v: &str) -> core::result::Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}

fn to_utf8(v: &[u8]) -> core::result::Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

pub fn open_and_parse_file(file_name: &str) -> core::result::Result<u32, CliError>  {
    // 重要：?
    let content = std::fs::read_to_string(&file_name)?;

    let num_str = to_utf8(content.as_bytes())?;
    println!("{:?}", num_str);

    let num = to_u32(&num_str)?;
    println!("{:?}", num);

    Ok(num)
}