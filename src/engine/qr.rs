use image::DynamicImage;

/// Scans image for QR codes using `bardecoder`.
///
/// Returns `Some(String)` with the decoded content if a valid QR code is found,
/// or `None` if no QR code was detected.
#[must_use]
pub fn detect_qr(img: &DynamicImage) -> Option<String> {
    let rgba = img.to_rgba8();
    let decoder = bardecoder::default_decoder();
    let results = decoder.decode(&rgba);

    for res in results.into_iter().flatten() {
        let trimmed = res.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    None
}
