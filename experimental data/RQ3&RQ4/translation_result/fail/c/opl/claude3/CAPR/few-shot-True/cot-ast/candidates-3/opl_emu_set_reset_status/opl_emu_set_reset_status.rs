use std::error::Error;

fn generate_server_response() -> Result<String, Box<dyn Error>> {
    // Simulating server response generation
    let response = "Server generated response".to_string();
    
    if response.is_empty() {
        Err("Failed to generate a response".into())
    } else {
        Ok(response)
    }
}
