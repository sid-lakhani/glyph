use image::imageops::FilterType;
use image::{DynamicImage, GrayImage, Luma};

/// Preprocesses the captured screen crop for maximum OCR accuracy.
///
/// Steps:
/// 1. Upscale low-resolution text if height or width is small (< 300px).
/// 2. Convert to grayscale.
/// 3. Apply contrast stretching / normalization.
#[must_use]
pub fn enhance_for_ocr(img: &DynamicImage) -> DynamicImage {
    let (w, h) = (img.width(), img.height());

    // 1. Upscale small snips using high-quality Lanczos3 filter
    let scaled = if h < 300 || w < 300 {
        let scale_factor = if h < 150 || w < 150 { 3 } else { 2 };
        img.resize(w * scale_factor, h * scale_factor, FilterType::Lanczos3)
    } else {
        img.clone()
    };

    // 2. Convert to 8-bit grayscale
    let gray = scaled.to_luma8();

    // 3. Contrast stretch / min-max normalization
    let enhanced_gray = stretch_contrast(gray);

    DynamicImage::ImageLuma8(enhanced_gray)
}

/// Normalizes pixel intensities across the full 0..255 dynamic range.
fn stretch_contrast(mut gray: GrayImage) -> GrayImage {
    let mut min_val = 255u8;
    let mut max_val = 0u8;

    for &Luma([p]) in gray.pixels() {
        if p < min_val {
            min_val = p;
        }
        if p > max_val {
            max_val = p;
        }
    }

    // Avoid division by zero or stretching already uniform images
    let range = f32::from(max_val.saturating_sub(min_val));
    if range > 10.0 {
        let min_f = f32::from(min_val);
        for Luma([p]) in gray.pixels_mut() {
            let normalized = (f32::from(*p) - min_f) / range * 255.0;
            *p = normalized.clamp(0.0, 255.0) as u8;
        }
    }

    gray
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;

    #[test]
    fn test_enhance_upscales_small_image() {
        let small = DynamicImage::ImageRgb8(RgbImage::new(50, 50));
        let enhanced = enhance_for_ocr(&small);
        assert!(enhanced.width() >= 150);
        assert!(enhanced.height() >= 150);
    }
}

