use notify_rust::Notification;
use std::process::Command;

/// Sends a desktop notification with character count and text preview.
///
/// If `silent` is true, this function is a complete no-op.
/// If `notify-rust` D-Bus connection fails, falls back gracefully to `notify-send`.
pub fn notify_snipped(text: &str, is_qr: bool, silent: bool) {
    if silent {
        return;
    }

    let char_count = text.chars().count();
    let title = if is_qr {
        format!("✦ QR Code Decoded ({char_count} chars)")
    } else {
        format!("✦ Text Snipped ({char_count} chars)")
    };

    let preview = if char_count > 120 {
        let snippet: String = text.chars().take(120).collect();
        format!("{snippet}...")
    } else {
        text.to_string()
    };

    // 1. Try notify-rust via D-Bus
    let rust_res = Notification::new()
        .appname("glyph")
        .summary(&title)
        .body(&preview)
        .icon("edit-copy")
        .timeout(3000)
        .show();

    if rust_res.is_err() {
        // 2. Fallback to notify-send subprocess
        let _ = Command::new("notify-send")
            .args([
                "-a",
                "glyph",
                "-i",
                "edit-copy",
                "-t",
                "3000",
                &title,
                &preview,
            ])
            .output();
    }
}
