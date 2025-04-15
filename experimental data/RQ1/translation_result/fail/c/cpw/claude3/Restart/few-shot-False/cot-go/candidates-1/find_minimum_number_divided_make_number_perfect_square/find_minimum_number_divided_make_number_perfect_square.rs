
struct ErrorResponse {
    error: String,
    message: String,
}

fn create_error_response() -> ErrorResponse {
    ErrorResponse {
        error: String::from("Internal Server Error"),
        message: String::from("An unexpected error occurred"),
    }
}
