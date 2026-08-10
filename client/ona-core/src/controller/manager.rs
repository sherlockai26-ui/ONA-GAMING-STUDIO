// manager.rs
// Administración de controladores conectados.

#[derive(Debug)]
pub struct Controller {
    pub player_id: u8,
    pub connected: bool,
}

pub struct ControllerManager {
    pub controllers: Vec<Controller>,
}

impl ControllerManager {
    pub fn new() -> Self {
        Self {
            controllers: Vec::new(),
        }
    }

    pub fn connect(&mut self) {
        let id = self.controllers.len() as u8 + 1;

        self.controllers.push(Controller {
            player_id: id,
            connected: true,
        });

        println!("Controller connected: Player {}", id);
    }

    pub fn count(&self) -> usize {
        self.controllers.len()
    }
}