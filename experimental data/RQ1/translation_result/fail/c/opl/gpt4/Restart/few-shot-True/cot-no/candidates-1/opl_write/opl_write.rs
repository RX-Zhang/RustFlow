
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Unauthorized;

impl fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unauthorized")
    }
}

impl Error for Unauthorized {}

fn create_unauthorized_error() -> Box<dyn Error> {
    Box::new(Unauthorized)
}
