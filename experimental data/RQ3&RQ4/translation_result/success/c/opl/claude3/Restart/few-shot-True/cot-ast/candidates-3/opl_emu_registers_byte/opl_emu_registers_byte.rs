use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct RequestError;

impl Error for RequestError {}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "请求错误")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Err(Box::new(RequestError))
}
