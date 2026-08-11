pub mod qr;
pub mod session;
pub mod network {
    pub mod http_server;
    pub mod websocket;
}
pub mod game_manager;
pub mod input;
pub mod launcher;
pub mod runtime;

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{BufRead, BufReader},
        net::TcpStream,
        path::{Path, PathBuf},
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use crate::{
        game_manager::{importer::import_game_from_dir, library::GameLibrary},
        input::{
            bridge::GameInputBridge,
            dispatcher::dispatch,
            events::{ButtonState, OnaButton, OnaInputEvent},
        },
        launcher::process::GameLauncher,
        runtime::lifecycle::GameLifecycleState,
    };

    #[test]
    fn infrastructure_imports_lists_launches_receives_input_and_closes_generic_game() {
        let root = unique_temp_dir();
        let source = root.join("external-package");
        let app_data = root.join("ona-data");
        fs::create_dir_all(&source).expect("source package directory should be created");

        let executable = create_test_executable(&source);
        let manifest = format!(
            r#"{{
                "manifestVersion": 1,
                "identity": {{
                    "id": "studio.test.infrastructure",
                    "name": "Infrastructure Test Game",
                    "version": "0.1.0",
                    "developer": "ONA Test Lab"
                }},
                "presentation": {{
                    "description": "Generic infrastructure verification package."
                }},
                "execution": {{
                    "executable": "{}",
                    "workingDirectory": ".",
                    "arguments": []
                }},
                "requirements": {{
                    "platform": "{}",
                    "architecture": "{}"
                }},
                "display": {{
                    "fullscreen": false
                }},
                "input": {{
                    "profile": "ona-standard-controller-v1"
                }}
            }}"#,
            executable,
            std::env::consts::OS,
            std::env::consts::ARCH
        );

        fs::write(source.join("game.json"), manifest).expect("manifest should be written");

        let library = GameLibrary::new(&app_data);
        let imported = import_game_from_dir(&library, &source, false)
            .expect("generic game package should import");
        assert_eq!(imported.id, "studio.test.infrastructure");
        assert!(imported.executable.is_file());

        let catalog = library
            .list_games()
            .expect("library should list installed games");
        assert_eq!(catalog.games.len(), 1);

        let bridge = GameInputBridge::start_localhost(0).expect("input bridge should start");
        let bridge_status = bridge.status();
        let address = bridge_status.address.clone();

        let receiver = thread::spawn(move || {
            let stream = TcpStream::connect(address).expect("test game should connect to bridge");
            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            reader
                .read_line(&mut line)
                .expect("test game should receive input");
            line
        });

        thread::sleep(Duration::from_millis(100));
        dispatch(
            OnaInputEvent::Button {
                player_id: 1,
                button: OnaButton::A,
                state: ButtonState::Down,
            },
            Some(&bridge),
        );

        let received = receiver.join().expect("receiver should finish");
        assert!(received.contains("\"button\":\"A\""));

        let launcher = GameLauncher::new();
        let status = launcher.launch(&imported).expect("game should launch");
        assert_eq!(status.state, GameLifecycleState::Running);
        assert!(status.pid.is_some());

        let closed = launcher
            .terminate()
            .expect("game should close through launcher");
        assert_eq!(closed.state, GameLifecycleState::Idle);

        let _ = fs::remove_dir_all(root);
    }

    #[cfg(windows)]
    fn create_test_executable(source: &Path) -> String {
        let name = "game.cmd";
        let path = source.join(name);
        fs::write(
            &path,
            "@echo off\r\nping 127.0.0.1 -n 30 > nul\r\nexit /b 0\r\n",
        )
        .expect("test executable should be written");
        name.to_string()
    }

    #[cfg(not(windows))]
    fn create_test_executable(source: &Path) -> String {
        use std::os::unix::fs::PermissionsExt;

        let name = "game.sh";
        let path = source.join(name);
        fs::write(&path, "#!/bin/sh\nsleep 30\n").expect("test executable should be written");
        let mut permissions = fs::metadata(&path)
            .expect("metadata should exist")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).expect("permissions should be set");
        name.to_string()
    }

    fn unique_temp_dir() -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();

        std::env::temp_dir().join(format!("ona-core-infra-test-{timestamp}"))
    }
}
