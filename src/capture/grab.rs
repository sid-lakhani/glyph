use image::DynamicImage;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrabError {
    #[error("`grim` command not found on $PATH. Please install grim (e.g. pacman -S grim)")]
    GrimNotFound(#[source] std::io::Error),
    #[error("Failed to execute `grim`: {0}")]
    ExecutionFailed(#[from] std::io::Error),
    #[error("`grim` capture failed: {0}")]
    GrimFailed(String),
    #[error("Failed to decode captured PNG image from memory: {0}")]
    ImageDecodeFailed(#[from] image::ImageError),
}

/// Grabs screen pixels for the given geometry using `grim` piped directly into memory.
pub fn grab_geometry(geometry: &str) -> Result<DynamicImage, GrabError> {
    let output = Command::new("grim")
        .args(["-g", geometry, "-t", "png", "-"])
        .output()
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                GrabError::GrimNotFound(err)
            } else {
                GrabError::ExecutionFailed(err)
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GrabError::GrimFailed(stderr.trim().to_string()));
    }

    let img = image::load_from_memory(&output.stdout)?;
    Ok(img)
}

