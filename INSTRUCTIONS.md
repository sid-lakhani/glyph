# ✦ glyph — Agent Execution Guide & Implementation Tasks

> **For AI Agents:** This document outlines the step-by-step implementation milestones, development rules, verification procedures, and AUR packaging instructions for building `glyph`.

---

## 🛠 Project Rules & Standards

1. **Rust Standards:**
   - Edition 2021+.
   - Idiomatic, zero-slop Rust: strictly avoid unnecessary wrapper structs, premature abstractions, or empty trait implementations.
   - Zero `unwrap()` or `panic!()` in runtime paths. Return descriptive `Result<T, E>` types.
   - Code must pass `cargo clippy -- -D warnings` and `cargo fmt --check`.

2. **Unix & Wayland Integrity:**
   - Must handle graceful user cancellations (e.g. user hits `Escape` during region selection &rarr; exit 0 quietly, don't spam errors).
   - In-memory streaming: avoid writing intermediate screenshot files to disk (`/tmp` or home directory). Keep byte streams in memory.

---

## 📋 Implementation Checklist

### Milestone 1: Core Scaffolding
- [ ] Initialize git repository if not already tracked.
- [ ] Set up directory structure:
  ```
  src/
  ├── capture/       # Region selection & pixel acquisition
  │   ├── mod.rs
  │   ├── region.rs
  │   └── grab.rs
  ├── preprocess/    # Image enhancement (contrast, binarization, scale)
  │   └── mod.rs
  ├── engine/        # OCR & QR detection engines
  │   ├── mod.rs
  │   ├── tesseract.rs
  │   └── qr.rs
  ├── cleaner/       # Text formatting (prose, code, line modes)
  │   └── mod.rs
  ├── output/        # Wayland clipboard & desktop notifications
  │   ├── mod.rs
  │   ├── clipboard.rs
  │   └── notify.rs
  ├── cli.rs         # Clap command-line parser
  └── main.rs        # Orchestration pipeline
  ```

### Milestone 2: Capture Engine
- [ ] Implement `src/capture/region.rs`:
  - Run `slurp -f "%x %y %w %h"` or `slurp -b 00000088 -c b8f3f1 -s 00000000 -w 2`.
  - Capture geometry string; handle non-zero exit codes (user cancelled) cleanly.
- [ ] Implement `src/capture/grab.rs`:
  - Pipe geometry into `grim -g "<geom>" -t png -` and read raw PNG bytes directly into `image::load_from_memory`.

### Milestone 3: Image Enhancement & OCR Engine
- [ ] Implement `src/preprocess/mod.rs`:
  - Upscale small crops (`< 300px` height) with Lanczos3 filter.
  - Convert to grayscale, apply contrast stretching to make text boundaries crisp.
- [ ] Implement `src/engine/qr.rs`:
  - Run QR code detection via `bardecoder`. If found, return decoded payload immediately.
- [ ] Implement `src/engine/tesseract.rs`:
  - Interface with `leptess` (or subprocess fallback if C library is missing).
  - Extract text and confidence scores.

### Milestone 4: Smart Text Cleaner
- [ ] Implement `src/cleaner/mod.rs`:
  - **Prose mode (default):** remove accidental linebreaks within paragraphs, fix hyphenated splits.
  - **Code mode (`--code`):** preserve whitespace indentation, tabs, code syntax symbols.
  - **Single-line mode (`--line`):** strip all internal newlines and multiple spaces into a single space.

### Milestone 5: Output & Notifications
- [ ] Implement `src/output/clipboard.rs`:
  - Copy clean text to clipboard using `arboard` (or fallback to `wl-copy`).
- [ ] Implement `src/output/notify.rs`:
  - Trigger desktop notification with character count and preview.
  - Respect `--silent` flag.

### Milestone 6: AUR Packaging & GitHub Release
- [ ] Write `PKGBUILD` for Arch Linux (`aur/glyph-bin` and `aur/glyph-git`).
- [ ] Write `.github/workflows/release.yml` with cross-compiled automated binary releases for Linux x86_64.
- [ ] Test integration into Hyprland dotfiles:
  ```ini
  bind = Super+Shift, T, exec, glyph
  bind = Super+Shift+Alt, T, exec, glyph --code
  ```

---

## 🧪 Verification Commands

When testing changes, run:
```bash
# 1. Check compiler warnings & formatting
cargo check
cargo clippy -- -D warnings
cargo fmt --check

# 2. Test execution locally
cargo run -- --help
cargo run -- --stdout
```
