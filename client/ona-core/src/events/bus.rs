// bus.rs
// Bus interno de eventos ONA Core.
// Permite comunicación entre módulos sin acoplamiento directo.

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct EventBus {
    events: Arc<Mutex<Vec<String>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn publish(&self, event: &str) {
        let mut events = self.events.lock().unwrap();
        events.push(event.to_string());

        println!("EVENT: {}", event);
    }

    pub fn consume(&self) -> Vec<String> {
        let mut events = self.events.lock().unwrap();

        let result = events.clone();
        events.clear();

        result
    }
}