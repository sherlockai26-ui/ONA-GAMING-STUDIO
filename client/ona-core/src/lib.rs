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
        process::Command,
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use crate::{
        game_manager::{
            importer::import_game_from_dir,
            library::GameLibrary,
            scanner::{scan_game_packages, InstallationSource, InstallationSourceKind},
        },
        input::{
            bridge::GameInputBridge,
            dispatcher::dispatch,
            events::{ButtonState, OnaButton, OnaInputEvent},
        },
        launcher::process::{GameLauncher, OnaGameRuntimeContext},
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

    #[test]
    fn launcher_passes_ona_runtime_v1_environment_to_child_process() {
        let root = unique_temp_dir();
        let source = root.join("runtime-env-package");
        let app_data = root.join("ona-data");
        let env_output = root.join("runtime-env.txt");
        fs::create_dir_all(&source).expect("source package directory should be created");

        let executable = create_env_capture_executable(&source, &env_output);
        let manifest = format!(
            r#"{{
                "manifestVersion": 1,
                "identity": {{
                    "id": "studio.test.runtime-env",
                    "name": "Runtime Env Test Game",
                    "version": "0.1.0",
                    "developer": "ONA Test Lab"
                }},
                "presentation": {{
                    "description": "Runtime environment verification package."
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
                "input": {{
                    "profile": "ona-standard-controller-v1"
                }},
                "display": {{
                    "fullscreen": false
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
        let bridge = GameInputBridge::start_localhost(0).expect("input bridge should start");
        let bridge_status = bridge.status();
        let launcher = GameLauncher::new();

        let status = launcher
            .launch_with_runtime(
                &imported,
                OnaGameRuntimeContext {
                    input_host: bridge_status.host.clone(),
                    input_port: bridge_status.port,
                    player_id: None,
                },
            )
            .expect("game should launch with runtime context");

        assert_eq!(status.state, GameLifecycleState::Running);

        let mut captured = String::new();

        for _ in 0..40 {
            captured = fs::read_to_string(&env_output).unwrap_or_default();

            if captured.contains("ONA_RUNTIME=1") {
                break;
            }

            thread::sleep(Duration::from_millis(50));
        }

        assert!(captured.contains("ONA_RUNTIME=1"));
        assert!(captured.contains("ONA_PROTOCOL_VERSION=1"));
        assert!(captured.contains(&format!("ONA_INPUT_HOST={}", bridge_status.host)));
        assert!(captured.contains(&format!("ONA_INPUT_PORT={}", bridge_status.port)));

        let _ = launcher.terminate();
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn scanner_reports_valid_invalid_multiple_and_already_installed_packages() {
        let root = unique_temp_dir();
        let app_data = root.join("ona-data");
        let source_root = root.join("external-source");
        let ona_library = source_root.join("ONA Library");
        let valid_one = ona_library.join("GAME-ONE");
        let valid_two = ona_library.join("GAME-TWO");
        let invalid = ona_library.join("BROKEN-GAME");

        fs::create_dir_all(&valid_one).expect("valid package should be created");
        fs::create_dir_all(&valid_two).expect("valid package should be created");
        fs::create_dir_all(&invalid).expect("invalid package should be created");

        let valid_one_executable = create_test_executable(&valid_one);
        let valid_two_executable = create_test_executable(&valid_two);

        write_test_manifest(
            &valid_one,
            "studio.test.one",
            "Scanner Game One",
            &valid_one_executable,
        );
        write_test_manifest(
            &valid_two,
            "studio.test.two",
            "Scanner Game Two",
            &valid_two_executable,
        );
        fs::write(invalid.join("game.json"), r#"{"identity":{"id":"broken"}}"#)
            .expect("invalid manifest should be written");

        let library = GameLibrary::new(&app_data);
        import_game_from_dir(&library, &valid_one, false)
            .expect("first package should install for duplicate detection");

        let report = scan_game_packages(
            &library,
            vec![InstallationSource {
                id: "TEST".to_string(),
                name: "USB DRIVE TEST".to_string(),
                root: source_root,
                library_path: ona_library,
                kind: InstallationSourceKind::ExternalStorage,
            }],
        );

        assert_eq!(report.sources.len(), 1);
        assert_eq!(report.games.len(), 2);
        assert_eq!(report.invalid_packages.len(), 1);
        assert!(report
            .games
            .iter()
            .any(|game| game.game_id == "studio.test.one" && game.already_installed));
        assert!(report
            .games
            .iter()
            .any(|game| game.game_id == "studio.test.two" && !game.already_installed));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn uninstall_removes_only_managed_local_copy_and_keeps_external_package() {
        let root = unique_temp_dir();
        let source = root.join("external-package");
        let app_data = root.join("ona-data");
        fs::create_dir_all(&source).expect("source package directory should be created");

        let executable = create_test_executable(&source);
        write_test_manifest(
            &source,
            "studio.test.uninstall",
            "Uninstall Test Game",
            &executable,
        );

        let library = GameLibrary::new(&app_data);
        let imported = import_game_from_dir(&library, &source, false)
            .expect("generic game package should import");

        assert!(imported.install_dir.is_dir());
        assert!(source.join("game.json").is_file());

        library
            .uninstall_game("studio.test.uninstall")
            .expect("managed game should uninstall");

        assert!(!imported.install_dir.exists());
        assert!(source.join("game.json").is_file());
        assert!(source.join(&executable).is_file());
        assert!(library
            .list_games()
            .expect("library should list after uninstall")
            .games
            .is_empty());

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

    #[cfg(windows)]
    fn create_env_capture_executable(source: &Path, output: &Path) -> String {
        let source_file = source.join("capture-env.rs");
        let executable = source.join("capture-env.exe");
        let output = output.to_string_lossy().replace('\\', "\\\\");

        fs::write(
            &source_file,
            format!(
                "fn main() {{
                    let output = r#\"{output}\"#;
                    let content = format!(
                        \"ONA_RUNTIME={{}}\\nONA_PROTOCOL_VERSION={{}}\\nONA_INPUT_HOST={{}}\\nONA_INPUT_PORT={{}}\\n\",
                        std::env::var(\"ONA_RUNTIME\").unwrap_or_default(),
                        std::env::var(\"ONA_PROTOCOL_VERSION\").unwrap_or_default(),
                        std::env::var(\"ONA_INPUT_HOST\").unwrap_or_default(),
                        std::env::var(\"ONA_INPUT_PORT\").unwrap_or_default()
                    );
                    std::fs::write(output, content).unwrap();
                }}"
            ),
        )
        .expect("test source should be written");

        let status = Command::new("rustc")
            .arg(&source_file)
            .arg("-o")
            .arg(&executable)
            .status()
            .expect("rustc should run");

        assert!(status.success(), "test executable should compile");

        "capture-env.exe".to_string()
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

    #[cfg(not(windows))]
    fn create_env_capture_executable(source: &Path, output: &Path) -> String {
        use std::os::unix::fs::PermissionsExt;

        let name = "capture-env.sh";
        let path = source.join(name);
        let output = output.to_string_lossy();

        fs::write(
            &path,
            format!(
                "#!/bin/sh\n\
                echo ONA_RUNTIME=$ONA_RUNTIME > \"{output}\"\n\
                echo ONA_PROTOCOL_VERSION=$ONA_PROTOCOL_VERSION >> \"{output}\"\n\
                echo ONA_INPUT_HOST=$ONA_INPUT_HOST >> \"{output}\"\n\
                echo ONA_INPUT_PORT=$ONA_INPUT_PORT >> \"{output}\"\n"
            ),
        )
        .expect("test executable should be written");

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

    fn write_test_manifest(package_dir: &Path, id: &str, name: &str, executable: &str) {
        let manifest = format!(
            r#"{{
                "manifestVersion": 1,
                "identity": {{
                    "id": "{id}",
                    "name": "{name}",
                    "version": "0.1.0",
                    "developer": "ONA Test Lab"
                }},
                "presentation": {{
                    "description": "Scanner verification package."
                }},
                "execution": {{
                    "executable": "{executable}",
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
            std::env::consts::OS,
            std::env::consts::ARCH
        );

        fs::write(package_dir.join("game.json"), manifest).expect("manifest should be written");
    }
}
