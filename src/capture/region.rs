use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegionError {
    #[error("`slurp` command not found on $PATH. Please install slurp (e.g. pacman -S slurp)")]
    SlurpNotFound(#[source] std::io::Error),
    #[error("Failed to execute `slurp`: {0}")]
    ExecutionFailed(#[from] std::io::Error),
    #[error("Slurp produced invalid UTF-8 output")]
    InvalidOutput,
}

/// Invokes `slurp` to let user select a screen region.
///
/// Returns `Ok(Some(geometry))` if selected, or `Ok(None)` if the user cancelled (e.g. pressed Escape).
pub fn select_region() -> Result<Option<String>, RegionError> {
    let output = Command::new("slurp")
        .args([
            "-b", "00000088", "-c", "b8f3f1", "-s", "00000000", "-w", "2",
        ])
        .output()
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                RegionError::SlurpNotFound(err)
            } else {
                RegionError::ExecutionFailed(err)
            }
        })?;

    // Slurp exits with code 1 if user hits Escape or clicks outside
    if !output.status.success() {
        return Ok(None);
    }

    let geom = String::from_utf8(output.stdout)
        .map_err(|_| RegionError::InvalidOutput)?
        .trim()
        .to_string();

    if geom.is_empty() {
        Ok(None)
    } else {
        Ok(Some(geom))
    }
}
