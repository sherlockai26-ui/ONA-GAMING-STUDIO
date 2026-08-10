// process.rs
// Lanzador básico de juegos.

use std::process::Command;

pub fn launch(path: &str) {

    println!("Launching game:");
    println!("{}", path);

    let result = Command::new(path)
        .spawn();

    match result {
        Ok(_) => println!("Game started."),
        Err(e) => println!("Launch error: {}", e),
    }
}