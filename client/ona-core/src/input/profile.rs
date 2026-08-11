use serde::{Deserialize, Serialize};

use super::{calibration::StickCalibration, events::OnaButton};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnaControllerProfile {
    pub name: String,
    pub stick: StickCalibration,
    pub button_mapping: Vec<ButtonMapping>,
}

impl Default for OnaControllerProfile {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            stick: StickCalibration::default(),
            button_mapping: default_button_mapping(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonMapping {
    pub physical: OnaButton,
    pub ona_action: OnaButton,
}

fn default_button_mapping() -> Vec<ButtonMapping> {
    [
        OnaButton::A,
        OnaButton::B,
        OnaButton::X,
        OnaButton::Y,
        OnaButton::L1,
        OnaButton::L2,
        OnaButton::R1,
        OnaButton::R2,
        OnaButton::Select,
        OnaButton::Start,
    ]
    .into_iter()
    .map(|button| ButtonMapping {
        physical: button.clone(),
        ona_action: button,
    })
    .collect()
}
