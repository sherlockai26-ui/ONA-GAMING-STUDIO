// token.rs
// Generación y manejo de tokens temporales para emparejamiento QR.
// MVP: token simple en memoria.
// Futuro: expiración, firma criptográfica y validación avanzada.

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct SessionToken {
    pub value: String,
    pub created_at: u64,
}

impl SessionToken {
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let value = format!("ONA-{}", timestamp);

        Self {
            value,
            created_at: timestamp,
        }
    }

    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Validez temporal MVP: 5 minutos
        now - self.created_at < 300
    }
}
