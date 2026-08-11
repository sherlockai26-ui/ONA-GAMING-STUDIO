use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickCalibration {
    pub center_x: f32,
    pub center_y: f32,
    pub deadzone: f32,
    pub sensitivity: f32,
}

impl Default for StickCalibration {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_y: 0.0,
            deadzone: 0.12,
            sensitivity: 1.0,
        }
    }
}

impl StickCalibration {
    pub fn calibrated(self, x: f32, y: f32) -> (f32, f32) {
        let adjusted_x = x - self.center_x;
        let adjusted_y = y - self.center_y;
        let distance = (adjusted_x * adjusted_x + adjusted_y * adjusted_y).sqrt();

        if distance <= self.deadzone {
            return (0.0, 0.0);
        }

        let range = 1.0 - self.deadzone;
        let scaled_distance = ((distance - self.deadzone) / range).clamp(0.0, 1.0);
        let scale = if distance > 0.0 {
            scaled_distance / distance
        } else {
            0.0
        };

        (
            (adjusted_x * scale * self.sensitivity).clamp(-1.0, 1.0),
            (adjusted_y * scale * self.sensitivity).clamp(-1.0, 1.0),
        )
    }
}
