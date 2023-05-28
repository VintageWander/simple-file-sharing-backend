use validator::ValidationErrors;

pub fn extract_validation_error(e: &ValidationErrors) -> String {
    let mut message = "".to_string();
    message.insert(0, 'F');
    for field in e.field_errors() {
        message.insert_str(message.len(), format!("ield {}: ", field.0).as_str());
        for field_error in field.1 {
            if let Some(msg) = &field_error.message {
                message.insert_str(message.len(), format!("{}; F", msg).as_str())
            } else {
                message.insert_str(message.len(), format!("{}; F", field_error.code).as_str())
            }
        }
    }
    message[0..message.len() - 3].to_string()
}
