# ✦ glyph — Architecture & Technical Specification

## 1. Core Principles
1. **Wayland First:** First-class citizen for Hyprland, Sway, and wlroots compositors. No X11 runtime dependencies.
2. **Instant Response (< 150ms):** Zero interpreter startup latency. Heavy initialization (like model loading or dictionary parsing) is minimized or lazily streamed.
3. **Pristine Text Quality:** Raw OCR often produces fragmented text (broken hyphenations, accidental spaces inside words, missing indentations). `glyph` must post-process text intentionally before touching the clipboard.
4. **Resilient Fallbacks:** If native Wayland capture protocol fails or fractional scaling produces mismatched geometry, gracefully fall back to composable helpers (`grim`/`slurp`) without panic.

---

## 2. Component Pipeline

### A. Region Selection (`src/capture/region.rs`)
- Invokes Wayland region selection (either via `slurp` process integration or Wayland layer-shell overlay).
- Must handle:
  - Multi-monitor setups with distinct scale factors (e.g. 1.0x on external, 1.25x on laptop).
  - User cancellation (Esc / click outside) &rarr; exit 0 silently without throwing error notifications.

### B. Frame Grabber (`src/capture/grab.rs`)
- Grabs raw pixels for the selected geometry `(x, y, w, h)`.
- Fallback hierarchy:
  1. `wlr-screencopy-v1` / `ext-image-copy-capture-v1`
  2. Subprocess `grim -g "<geom>" -` streaming directly into memory buffer (zero disk I/O).

### C. Image Enhancement (`src/preprocess/mod.rs`)
Low-resolution fonts, anti-aliased UI text, and dark-mode terminal fonts frequently cause OCR hallucination.
Before feeding into the OCR engine:
- Convert to Grayscale (`8-bit luma`).
- Apply adaptive thresholding or Otsu binarization.
- Bicubic / Lanczos3 upscale by 2x for small text regions (< 300px height).
- Contrast normalization to maximize character boundary definition.

### D. Recognition Engine (`src/engine/mod.rs`)
Modular trait architecture:
```rust
pub trait OcrEngine {
    fn recognize(&self, img: &image::DynamicImage, lang: &str) -> anyhow::Result<String>;
}
```
- **Backend 1 (Default):** `Tesseract` via C ABI / `leptess`. Standard, battle-tested, available in every distro (`extra/tesseract`).
- **Backend 2 (Fast QR):** `bardecoder` runs parallel check on the image. If a QR code is detected, bypasses OCR and returns the decoded string immediately.

### E. Text Cleaner & Formatter (`src/cleaner/mod.rs`)
- **Prose Mode (Default):**
  - Un-wraps hard line breaks that split sentences across lines.
  - Re-joins hyphenated line breaks (`exam-\nple` &rarr; `example`).
  - Trims trailing whitespace.
- **Code Mode (`--code`):**
  - Preserves leading indentation (spaces/tabs).
  - Preserves brackets and semicolons.
  - Normalizes quotes (`“` / `”` &rarr; `"`).
- **Line Mode (`--line`):**
  - Collapses all newlines and multiple spaces into a single clean line.

### F. Clipboard & Presentation (`src/output/mod.rs`)
- **Clipboard:** Native Wayland Data Control via `arboard` (`wl-copy` fallback).
- **Notification:** Formatted desktop notification via `notify-rust` (Z-Bus):
  - Summary: `✦ Text Snipped (N chars)`
  - Body: Truncated preview (first 120 characters with ellipsis).
  - Icon: `edit-copy` or `accessories-character-map`.

---

## 3. CLI Design & Flags

```text
Usage: glyph [OPTIONS]

Options:
  -c, --code            Preserve indentation and code structure
  -l, --line            Collapse text into a single continuous line
  -s, --silent          Suppress desktop notification
      --stdout          Print recognized text directly to stdout (pipe friendly)
      --lang <LANG>     OCR language code (default: "eng")
  -h, --help            Print help
  -V, --version         Print version
```

---

## 4. Error Handling
- Never crash with an unhandled `unwrap()` or `expect()`.
- Use `thiserror` for typed domain errors (`CaptureError`, `OcrError`, `ClipboardError`).
- Use `anyhow` in `main.rs` for user-facing exit messaging.
