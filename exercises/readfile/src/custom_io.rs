#[derive(Debug)]
pub struct ReadFileError {
    pub message: String
}

impl std::fmt::Display for ReadFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReadFileError: {}", self.message)
    }
}

impl std::error::Error for ReadFileError {}

pub type Result<T> = std::result::Result<T, ReadFileError>;


pub fn read_file(path: &str) -> Result<String> {  
    use std::io;
    let file: io::Result<String> = std::fs::read_to_string(path);

    match file {
        Ok(content) => Result::Ok(content),
        Err(err) => Result::Err(ReadFileError{message: err.to_string()}), // io::Error -> ReadFileError
    }
}