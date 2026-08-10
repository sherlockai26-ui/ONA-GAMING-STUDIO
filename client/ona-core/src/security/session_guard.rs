// session_guard.rs
// Validación básica de sesiones MVP.

pub fn validate_token(token: &str) -> bool {

    if token.starts_with("ONA-") {
        return true;
    }

    false
}