use config::settings;
use core::time::Duration;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::thread;
use test_suite::error::HarnessError;
use tokio::time::Instant;
use tracing::error;

pub const PROJECT_NAME: &str = "vms";

fn main() -> ExitCode {
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
    compose_cmd(["--profile", "test", "build", "test-suite"])?;
    let result = compose_cmd(["--profile", "test", "run", "--rm", "test-suite"]);
    // let down_result = docker_compose_down();
    result?;
    Ok(())
    // down_result
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
            return Err(HarnessError::DockerError(format!(
                "docker is installed but not usable (status: {status}); ensure Docker daemon is running"
            )));
        }
        Err(e) => {
            return Err(HarnessError::DockerError(format!(
                "docker is not installed or not in PATH: {e}"
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
        Ok(_) => Err(HarnessError::DockerError(
            "Docker is installed but `docker compose` is unavailable; install Docker Compose v2 plugin"
        .to_string())),
        Err(e) => Err(HarnessError::DockerError(format!(
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
        Ok(_) => Err(HarnessError::DockerError(
            "Docker daemon is not reachable".to_string(),
        )),
        Err(e) => Err(HarnessError::DockerError(format!(
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
    let server_addr = format!("{bind_addr}:{server_port}");
    // let server_addr = helpers::build_server_addr(login_addr, server_port);
    wait_for_endpoint(&server_addr, Duration::from_secs(60))?;
    Ok(())
}

fn docker_compose_down() -> Result<(), HarnessError> {
    let docker_stop = Command::new("sh")
        .args(["-c", "docker stop $(docker ps -aq) || true"])
        .status();
    match docker_stop {
        Ok(status) if status.success() => {}
        Ok(status) => return Err(HarnessError::DockerError(format!("stop error: {status}"))),
        Err(e) => return Err(HarnessError::DockerError(format!("stop error: {e}"))),
    }
    compose_cmd(["down", "--remove-orphans"])?;
    let docker_rm = Command::new("docker").args(["volume", "rm", "db"]).status();
    match docker_rm {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(HarnessError::DockerError(format!("rm db error: {status}"))),
        Err(e) => Err(HarnessError::DockerError(format!("rm db error: {e}"))),
    }
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
        .map_err(|e| HarnessError::DockerError(format!("Failed to execute docker compose: {e}")))?;

    if status.success() {
        Ok(())
    } else {
        Err(HarnessError::DockerError(format!(
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
