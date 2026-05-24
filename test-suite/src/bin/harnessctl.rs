use config::error::ConfigError;
use config::settings;
use core::time::Duration;
use inc::helpers;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::{io, thread};
use tokio::time::Instant;
use tracing::error;
use tracing_subscriber::EnvFilter;

pub const PROJECT_NAME: &str = "vms";

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("vms=debug".parse().unwrap()))
        .init();
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            error!("test_suite error: {}", e.to_string());
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    ensure_docker_available()?;
    run_tests()
}

fn get_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("test_suite should be in workspace root")
}

fn run_tests() -> Result<(), String> {
    if let Err(e) = docker_compose_up() {
        let _ = docker_compose_down();
        return Err(e);
    }
    let root: &'static Path = get_workspace_root();
    let status = Command::new("cargo")
        .arg("test")
        .arg("-p")
        .arg("test_suite")
        .current_dir(&root)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("failed to execute cargo test: {e}"))?;
    let down_result = docker_compose_down();
    if !status.success() {
        return Err(format!("cargo test failed with status {status}"));
    }
    down_result
}

fn ensure_docker_available() -> Result<(), String> {
    let docker_status = Command::new("docker")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match docker_status {
        Ok(status) if status.success() => {}
        Ok(status) => {
            return Err(format!(
                "docker is installed but not usable (status: {status}); ensure Docker daemon is running"
            ));
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Err(
                "docker is not installed or not in PATH; install Docker Engine and Docker Compose"
                    .to_string(),
            );
        }
        Err(error) => return Err(format!("failed to execute `docker --version`: {error}")),
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
        Ok(_) => Err(
            "docker is installed but `docker compose` is unavailable; install Docker Compose v2 plugin"
                .to_string(),
        ),
        Err(error) => Err(format!("failed to execute `docker compose version`: {error}")),
    }?;
    let daemon_status = Command::new("docker")
        .arg("info")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match daemon_status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(
            "docker daemon is not reachable; start the Docker service and ensure your user can access /var/run/docker.sock"
                .to_string(),
        ),
        Err(error) => Err(format!("failed to execute `docker info`: {error}")),
    }
}

fn docker_compose_up() -> Result<(), String> {
    let _ = docker_compose_down();
    compose_cmd(["build", "-q"])?;
    compose_cmd(["up", "-d", "--no-build", "--remove-orphans"])?;
    let bind_addr: String = match settings::get_bind_address() {
        Ok(addr) => addr,
        Err(_) => return Err(format!("Failed to retrieve server addess")),
    };
    let server_port: i16 = match settings::get_login_port() {
        Ok(port) => port,
        Err(_) => return Err(format!("Failed to retrieve server port")),
    };
    let server_addr = String::from(format!("{bind_addr}:{server_port}"));
    // let server_addr = helpers::build_server_addr(login_addr, server_port);
    wait_for_endpoint(&server_addr, Duration::from_secs(60))?;
    Ok(())
}

fn docker_compose_down() -> Result<(), String> {
    compose_cmd(["down", "-v", "--remove-orphans"])
}

fn compose_cmd<const N: usize>(args: [&str; N]) -> Result<(), String> {
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
        .map_err(|error| format!("failed to execute docker compose: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "docker compose command failed with status {status}"
        ))
    }
}

fn compose_file_path() -> PathBuf {
    get_workspace_root().join("docker-compose.yml")
}

fn wait_for_endpoint(addr: &str, timeout: Duration) -> Result<(), String> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        match TcpStream::connect(addr) {
            Ok(_) => return Ok(()),
            Err(error) if error.kind() == io::ErrorKind::ConnectionRefused => {
                thread::sleep(Duration::from_millis(250));
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(250));
            }
        }
    }

    Err(format!(
        "timed out waiting for endpoint {addr} after {}s",
        timeout.as_secs()
    ))
}
