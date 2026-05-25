use config::error::ConfigError;
use config::settings;
use core::time::Duration;
use dotenvy;
use inc::helpers;
use state::model::SharedState;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::{io, thread};
use test_suite::error::HarnessError;
use tokio::time::Instant;
use tracing::error;
use tracing_subscriber::EnvFilter;

pub const PROJECT_NAME: &str = "vms";

fn main() -> ExitCode {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("vms=debug".parse().unwrap()))
        .init();
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            error!("test-suite error: {}", e.to_string());
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), HarnessError> {
    ensure_docker_available()?;
    run_tests()
}

fn get_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("test-suite should be in workspace root")
}

fn run_tests() -> Result<(), HarnessError> {
    if let Err(e) = docker_compose_up() {
        let _ = docker_compose_down();
        return Err(e);
    }
    let root: &'static Path = get_workspace_root();
    let status = Command::new("cargo")
        .arg("test")
        .arg("-p")
        .arg("test-suite")
        .current_dir(&root)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| HarnessError::CargoError(format!("Cargo test exited with an error: {e}")))?;
    let down_result = docker_compose_down();
    if !status.success() {
        return Err(HarnessError::CargoError(format!(
            "Cargo test exited with a failed status: {status}"
        )));
    }
    down_result
}

fn ensure_docker_available() -> Result<(), HarnessError> {
    let docker_status = Command::new("docker")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match docker_status {
        Ok(status) if status.success() => {}
        Ok(status) => {
            return Err(HarnessError::DockerVersionError(format!(
                "docker is installed but not usable (status: {status}); ensure Docker daemon is running"
            )));
        }
        Err(error) => {
            return Err(HarnessError::DockerVersionError(format!(
                "docker is not installed or not in PATH"
            )));
        }
    }
    let compose_status = Command::new("docker")
        .arg("compose")
        .arg("version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match compose_status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(HarnessError::DockerComposeError(format!(
            "Docker is installed but `docker compose` is unavailable; install Docker Compose v2 plugin"
        ))),
        Err(e) => Err(HarnessError::DockerComposeError(format!(
            "Failed to execute `docker compose version`: {e}"
        ))),
    }?;
    let daemon_status = Command::new("docker")
        .arg("info")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match daemon_status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(HarnessError::DockerInfoError(
            "Docker daemon is not reachable".to_string(),
        )),
        Err(e) => Err(HarnessError::DockerInfoError(format!(
            "Failed to execute `docker info`: {e}"
        ))),
    }
}

fn docker_compose_up() -> Result<(), HarnessError> {
    let _ = docker_compose_down();
    compose_cmd(["build", "-q"])?;
    compose_cmd(["up", "-d", "--no-build", "--remove-orphans"])?;
    let bind_addr: String = settings::get_bind_address()?;
    let server_port: i16 = settings::get_login_port()?;
    let server_addr = String::from(format!("{bind_addr}:{server_port}"));
    // let server_addr = helpers::build_server_addr(login_addr, server_port);
    wait_for_endpoint(&server_addr, Duration::from_secs(60))?;
    Ok(())
}

fn docker_compose_down() -> Result<(), HarnessError> {
    compose_cmd(["down", "-v", "--remove-orphans"])
}

fn compose_cmd<const N: usize>(args: [&str; N]) -> Result<(), HarnessError> {
    let status = Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg(compose_file_path())
        .arg("-p")
        .arg(PROJECT_NAME)
        .args(args)
        .current_dir(get_workspace_root())
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| {
            HarnessError::DockerComposeError(format!("Failed to execute docker compose: {e}"))
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(HarnessError::DockerComposeError(format!(
            "Docker compose command failed with status {status}"
        )))
    }
}

fn compose_file_path() -> PathBuf {
    get_workspace_root().join("docker-compose.yml")
}

fn wait_for_endpoint(addr: &str, timeout: Duration) -> Result<(), HarnessError> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        match TcpStream::connect(addr) {
            Ok(_) => return Ok(()),
            Err(_) => {
                thread::sleep(Duration::from_millis(250));
            }
        }
    }

    Err(HarnessError::EndpointError(format!(
        "timed out waiting for endpoint {addr} after {}s",
        timeout.as_secs()
    )))
}
