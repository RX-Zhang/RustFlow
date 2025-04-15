
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct RequestError {
    message: String,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request error: {}", self.message)
    }
}

impl Error for RequestError {}

fn handle_request() -> Result<(), Box<dyn Error>> {
    Err(Box::new(RequestError {
        message: "An error occurred during the request".to_string(),
    }))
}
