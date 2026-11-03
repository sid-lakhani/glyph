pub mod qr;
pub mod tesseract;

use anyhow::Result;
use image::DynamicImage;

/// Unified OCR recognition interface.
pub trait OcrEngine {
    fn recognize(&self, img: &DynamicImage, lang: &str) -> Result<String>;
}

/// Default Tesseract OCR engine backend.
#[derive(Debug, Default)]
pub struct TesseractEngine;

impl OcrEngine for TesseractEngine {
    fn recognize(&self, img: &DynamicImage, lang: &str) -> Result<String> {
        tesseract::recognize(img, lang)
    }
}
