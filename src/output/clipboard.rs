use std::io::Write;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Failed to copy to clipboard via native Wayland and `wl-copy` fallback: {0}")]
    CopyFailed(String),
}

/// Copies text to the Wayland system clipboard.
///
/// Tries `arboard` (Wayland data-control) first.
/// If that fails, falls back gracefully to `wl-copy`.
pub fn set_text(text: &str) -> Result<(), ClipboardError> {
    // 1. Try native arboard clipboard
    if let Ok(mut clip) = arboard::Clipboard::new() {
        if clip.set_text(text).is_ok() {
            return Ok(());
        }
    }

    // 2. Fallback to wl-copy subprocess
    copy_wl_copy(text).map_err(|e| ClipboardError::CopyFailed(e.to_string()))
}

fn copy_wl_copy(text: &str) -> Result<(), std::io::Error> {
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(std::io::Error::other(format!(
            "`wl-copy` exited with status {status}"
        )));
    }

    Ok(())
}
