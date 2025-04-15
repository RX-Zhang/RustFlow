
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RequestError {
    message: String,
}

impl RequestError {
    pub fn new(message: &str) -> Self {
        RequestError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request error: {}", self.message)
    }
}

impl Error for RequestError {}

pub fn handle_request_error(error: RequestError) -> Result<(), Box<dyn Error>> {
    Err(Box::new(error))
}
