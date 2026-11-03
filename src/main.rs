mod capture;
mod cleaner;
mod cli;
mod engine;
mod output;
mod preprocess;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use engine::OcrEngine;
use tracing::{debug, info};

fn main() -> Result<()> {
    // Initialize tracing from RUST_LOG environment variable
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let cli = Cli::parse();
    debug!("Starting glyph with config: {:?}", cli);

    // 1. Capture screen region
    let img = match capture::capture_screen()? {
        Some(image) => image,
        None => {
            // User cancelled region selection (Esc / clicked outside)
            debug!("Region selection cancelled by user; exiting quietly.");
            return Ok(());
        }
    };

    // 2. Fast QR code detection check
    if let Some(qr_payload) = engine::qr::detect_qr(&img) {
        info!("Decoded QR code: {qr_payload}");
        if cli.stdout {
            println!("{qr_payload}");
        } else {
            output::clipboard::set_text(&qr_payload)?;
            output::notify::notify_snipped(&qr_payload, true, cli.silent);
        }
        return Ok(());
    }

    // 3. Image enhancement (upscaling small crops, grayscale, contrast normalization)
    let enhanced = preprocess::enhance_for_ocr(&img);

    // 4. OCR text recognition
    let ocr_engine = engine::TesseractEngine;
    let raw_text = ocr_engine.recognize(&enhanced, &cli.lang)?;

    // 5. Post-process & clean text
    let cleaned = cleaner::clean_text(&raw_text, cli.clean_mode());

    if cleaned.trim().is_empty() {
        if !cli.silent && !cli.stdout {
            output::notify::notify_snipped("No text detected in selected region", false, false);
        }
        return Ok(());
    }

    // 6. Output to stdout or clipboard + notification
    if cli.stdout {
        println!("{cleaned}");
    } else {
        output::clipboard::set_text(&cleaned)?;
        output::notify::notify_snipped(&cleaned, false, cli.silent);
    }

    Ok(())
}
