use std::io::Write;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Failed to copy to clipboard via `wl-copy` and `arboard`: {0}")]
    CopyFailed(String),
}

/// Copies text to the Wayland system clipboard.
///
/// On Wayland, short-lived CLI processes lose their clipboard offer upon exit
/// unless served by a resident clipboard daemon like `wl-copy`. Therefore,
/// `wl-copy` is used as the primary clipboard mechanism, with `arboard` as fallback.
pub fn set_text(text: &str) -> Result<(), ClipboardError> {
    // 1. Primary: wl-copy (forks into background to serve paste requests across Wayland)
    if copy_wl_copy(text).is_ok() {
        return Ok(());
    }

    // 2. Fallback: native arboard
    if let Ok(mut clip) = arboard::Clipboard::new() {
        if clip.set_text(text).is_ok() {
            return Ok(());
        }
    }

    Err(ClipboardError::CopyFailed(
        "Neither `wl-copy` nor `arboard` was able to set the clipboard.".to_string(),
    ))
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
