use anyhow::{bail, Context, Result};
use image::{DynamicImage, ImageFormat};
use std::io::{Cursor, Write};
use std::process::{Command, Stdio};

/// Runs Tesseract OCR on a DynamicImage using the specified language.
pub fn recognize(img: &DynamicImage, lang: &str) -> Result<String> {
    #[cfg(feature = "tesseract")]
    {
        recognize_native(img, lang)
    }

    #[cfg(not(feature = "tesseract"))]
    {
        recognize_subprocess(img, lang)
    }
}

#[cfg(feature = "tesseract")]
fn recognize_native(img: &DynamicImage, lang: &str) -> Result<String> {
    let mut png_bytes = Cursor::new(Vec::new());
    img.write_to(&mut png_bytes, ImageFormat::Png)
        .context("Failed to encode image to PNG for Leptess")?;

    let mut lt = leptess::LepTess::new(None, lang)
        .with_context(|| format!("Failed to initialize Tesseract with language '{lang}'"))?;

    lt.set_image_from_mem(png_bytes.get_ref())
        .context("Failed to pass image memory to Tesseract")?;

    let text = lt
        .get_utf8_text()
        .context("Failed to extract text from Tesseract")?;

    Ok(text)
}

#[cfg(not(feature = "tesseract"))]
fn recognize_subprocess(img: &DynamicImage, lang: &str) -> Result<String> {
    let mut png_bytes = Cursor::new(Vec::new());
    img.write_to(&mut png_bytes, ImageFormat::Png)
        .context("Failed to encode image to PNG for Tesseract subprocess")?;

    let mut child = Command::new("tesseract")
        .args(["stdin", "stdout", "-l", lang])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                anyhow::anyhow!(
                    "Tesseract OCR is not installed on this system.\n\
                     Please install it via your package manager:\n\
                       Arch:   sudo pacman -S tesseract tesseract-data-eng\n\
                       Fedora: sudo dnf install tesseract tesseract-langpack-eng\n\
                       Ubuntu: sudo apt install tesseract-ocr tesseract-ocr-eng"
                )
            } else {
                anyhow::anyhow!("Failed to spawn tesseract: {err}")
            }
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(png_bytes.get_ref())
            .context("Failed to pipe PNG image bytes to tesseract stdin")?;
    }

    let output = child
        .wait_with_output()
        .context("Failed while waiting for tesseract process to complete")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Tesseract OCR failed: {}", stderr.trim());
    }

    let text = String::from_utf8(output.stdout).context("Tesseract produced non-UTF8 output")?;

    Ok(text)
}
