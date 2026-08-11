// generator.rs
// Generador de URL para emparejamiento de controladores ONA.

use local_ip_address::local_ip;
use qrcode::{render::svg, QrCode};

pub fn generate(session_id: &str, token: &str, port: u16) -> String {
    let ip = match local_ip() {
        Ok(ip) => ip,
        Err(error) => {
            eprintln!("ERROR: Could not detect local IP address: {}", error);

            return format!(
                "http://localhost:{}/controller/?id={}&token={}",
                port, session_id, token
            );
        }
    };

    let url = format!(
        "http://{}:{}/controller/?id={}&token={}",
        ip, port, session_id, token
    );

    println!("QR URL generated:");
    println!("{}", url);

    url
}

pub fn generate_svg(content: &str, width: u32, height: u32) -> Result<String, String> {
    let code = QrCode::new(content.as_bytes())
        .map_err(|error| format!("Could not encode QR content: {error}"))?;

    Ok(code
        .render::<svg::Color>()
        .min_dimensions(width, height)
        .build())
}
