use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum OnaInputEvent {
    Joystick {
        player_id: u8,
        x: f32,
        y: f32,
    },
    Button {
        player_id: u8,
        button: OnaButton,
        state: ButtonState,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OnaButton {
    A,
    B,
    X,
    Y,
    L1,
    L2,
    R1,
    R2,
    Select,
    Start,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ButtonState {
    Down,
    Up,
    Pressed,
}

pub fn normalize_controller_json(raw: &str) -> Option<OnaInputEvent> {
    let value = serde_json::from_str::<serde_json::Value>(raw).ok()?;
    let player_id = value
        .get("playerId")
        .or_else(|| value.get("player_id"))
        .and_then(|value| value.as_u64())
        .unwrap_or(1) as u8;

    if value
        .get("control")
        .and_then(|value| value.as_str())
        .map(|control| control.eq_ignore_ascii_case("JOYSTICK"))
        .unwrap_or(false)
    {
        return Some(OnaInputEvent::Joystick {
            player_id,
            x: value
                .get("x")
                .and_then(|value| value.as_f64())
                .unwrap_or(0.0) as f32,
            y: value
                .get("y")
                .and_then(|value| value.as_f64())
                .unwrap_or(0.0) as f32,
        });
    }

    let button = value
        .get("button")
        .or_else(|| value.get("control"))
        .and_then(|value| value.as_str())
        .and_then(parse_button)?;

    let state = value
        .get("state")
        .and_then(|value| value.as_str())
        .and_then(parse_button_state)
        .unwrap_or(ButtonState::Pressed);

    Some(OnaInputEvent::Button {
        player_id,
        button,
        state,
    })
}

fn parse_button(value: &str) -> Option<OnaButton> {
    match value.to_ascii_uppercase().as_str() {
        "A" => Some(OnaButton::A),
        "B" => Some(OnaButton::B),
        "X" => Some(OnaButton::X),
        "Y" => Some(OnaButton::Y),
        "L1" => Some(OnaButton::L1),
        "L2" => Some(OnaButton::L2),
        "R1" => Some(OnaButton::R1),
        "R2" => Some(OnaButton::R2),
        "SELECT" => Some(OnaButton::Select),
        "START" => Some(OnaButton::Start),
        _ => None,
    }
}

fn parse_button_state(value: &str) -> Option<ButtonState> {
    match value.to_ascii_lowercase().as_str() {
        "down" => Some(ButtonState::Down),
        "up" => Some(ButtonState::Up),
        "pressed" => Some(ButtonState::Pressed),
        _ => None,
    }
}
