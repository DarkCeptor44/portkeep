use crate::server::api::v1::types::PortRequest;

/// Validate a port request
///
/// ## Arguments
///
/// * `payload` - The port request to validate
///
/// ## Returns
///
/// * `Result<(u16, String), String>` - The port and description if valid, or an error message
///
/// ## Errors
///
/// * `String` - If the port is invalid or the description is empty
pub fn validate_port(payload: PortRequest) -> Result<(u16, String), String> {
    if payload.port == 0 {
        return Err("Invalid port".to_string());
    }

    if payload.description.trim().is_empty() {
        return Err("Description cannot be empty".to_string());
    }

    Ok((payload.port, payload.description))
}
