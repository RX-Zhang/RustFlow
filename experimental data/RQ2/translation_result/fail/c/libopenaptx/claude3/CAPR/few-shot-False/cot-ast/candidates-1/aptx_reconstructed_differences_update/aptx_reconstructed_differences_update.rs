use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ServerError {
    message: String,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ServerError {}

fn server_response() -> Result<(), Box<dyn Error>> {
    Err(Box::new(ServerError {
        message: String::from("Server failed to generate a response")
    }))
}
