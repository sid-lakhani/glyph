# ✦ glyph

> **Blazing-fast, minimal Wayland screen OCR & text sniper.**  
> Drag a box anywhere &rarr; instant text on your clipboard + preview notification.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Wayland%20(Hyprland%20%2F%20Sway%20%2F%20KDE%20%2F%20GNOME)-brightgreen.svg)]()
[![Rust](https://img.shields.io/badge/Language-Rust%202021-orange.svg)]()

---

## ⚡ The Vision

`glyph` is the spiritual Wayland successor to [`pico`](https://github.com/sidlakhani/pico) (which was a minimal color picker built for X11). 

While `pico` targeted X11 right as the Linux ecosystem moved to Wayland, `glyph` is built **Wayland-native from day one**:
* **Zero UI Lag:** No bloated Electron/Python/GTK runtime. Written in pure Rust.
* **Smart Text Pipeline:** Automatically strips artificial newline breaks from PDF/doc copies while preserving real code indentation.
* **Auto QR & URL Recognition:** If you snip a QR code or web link, `glyph` detects it and gives you the decoded link directly.
* **High-Craft Polish:** Native desktop notification with character count and text preview, zero clipboard corruption.

---

## 🚀 Quick Usage

```bash
# Snip region & copy text to clipboard
glyph

# Snip with code indentation preserved (Code Mode)
glyph --code

# Snip a single line (strips all newlines/spaces)
glyph --line

# Output directly to stdout instead of clipboard (pipe friendly)
glyph --stdout

# Silent mode (no notifications)
glyph -s
```

---

## 📦 Architecture Overview

```
                      ┌──────────────────────┐
                      │    Screen Region     │
                      │  Selection (slurp)   │
                      └──────────┬───────────┘
                                 │
                                 ▼
                      ┌──────────────────────┐
                      │    Pixel Capture     │
                      │   (screencopy/grim)  │
                      └──────────┬───────────┘
                                 │
                                 ▼
                      ┌──────────────────────┐
                      │    Pre-processing    │
                      │ (contrast / upscale) │
                      └──────────┬───────────┘
                                 │
                 ┌───────────────┴───────────────┐
                 ▼                               ▼
       ┌───────────────────┐           ┌───────────────────┐
       │   OCR Engine      │           │  QR Code Decoder  │
       │ (Tesseract / ONNX)│           │   (bardecoder)    │
       └─────────┬─────────┘           └─────────┬─────────┘
                 │                               │
                 └───────────────┬───────────────┘
                                 │
                                 ▼
                      ┌──────────────────────┐
                      │    Post-Processor    │
                      │ (strip / format / rx)│
                      └──────────┬───────────┘
                                 │
                 ┌───────────────┴───────────────┐
                 ▼                               ▼
       ┌───────────────────┐           ┌───────────────────┐
       │  Wayland Data     │           │   Desktop Toast   │
       │ Control Clipboard │           │  (notify-rust)    │
       └───────────────────┘           └───────────────────┘
```

---

## 🛠️ Roadmap & Development

See [`ARCHITECTURE.md`](ARCHITECTURE.md) for detailed module architecture and technical specs.  
See [`INSTRUCTIONS.md`](INSTRUCTIONS.md) for step-by-step agent build instructions and task checklist.

---

## 📄 License

MIT © [Siddhesh Lakhani](https://github.com/sidlakhani)

