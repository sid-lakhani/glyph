# Contributing to glyph

Thank you for your interest in contributing to glyph. This document outlines development setup, coding standards, testing procedures, and the pull request workflow.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Development Setup](#development-setup)
- [Running Tests](#running-tests)
- [Code Style and Quality](#code-style-and-quality)
- [Architecture Guidelines](#architecture-guidelines)
- [Pull Request Process](#pull-request-process)
- [Reporting Issues](#reporting-issues)

---

## Prerequisites

glyph is built with Rust (Edition 2021) and targets Wayland desktop environments.

Ensure the following system tools and libraries are installed:

- **Rust Toolchain**: Stable Rust 1.75+ (`rustup`)
- **Screen Capture Utilities**: `slurp` and `grim`
- **Clipboard Utility**: `wl-clipboard` (`wl-copy`)
- **OCR Engine**: `tesseract` and development headers (`leptonica`, `libtesseract`)

### Package Manager Installation

**Arch Linux:**
```bash
sudo pacman -S --needed rustup slurp grim wl-clipboard tesseract tesseract-data-eng leptonica
```

**Fedora:**
```bash
sudo dnf install cargo slurp grim wl-clipboard tesseract-devel leptonica-devel tesseract-langpack-eng
```

**Ubuntu / Debian:**
```bash
sudo apt update
sudo apt install cargo slurp grim wl-clipboard libtesseract-dev libleptonica-dev tesseract-ocr-eng pkg-config
```

---

## Development Setup

1. Fork the repository on GitHub and clone your fork:
   ```bash
   git clone https://github.com/<your-username>/glyph.git
   cd glyph
   ```

2. Verify that the project builds cleanly:
   ```bash
   cargo build
   ```

3. To build with native C-ABI Leptess bindings:
   ```bash
   cargo build --features tesseract
   ```

---

## Running Tests

Run the automated test suite before submitting any changes:

```bash
cargo test
```

All unit tests must pass without regressions.

---

## Code Style and Quality

We enforce standard Rust formatting and zero compiler or Clippy warnings.

1. **Format Code:**
   ```bash
   cargo fmt
   ```
   Verify formatting:
   ```bash
   cargo fmt --check
   ```

2. **Run Linter:**
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Check Build:**
   ```bash
   cargo check --all-targets
   ```

---

## Architecture Guidelines

- **Zero-Allocation Streaming:** Avoid writing intermediate screenshots to temporary files on disk. Pixel buffers must be piped directly through memory.
- **Graceful Failure:** Never use unhandled `panic!()` or `unwrap()` in runtime execution paths. Return descriptive `Result<T, E>` types using `thiserror` for library domain errors and `anyhow` for top-level CLI error handling.
- **Clean Compositor Integration:** Cancel operations (such as pressing `Escape` during region selection) must exit with status `0` silently without raising error notifications.
- **Minimal Dependencies:** Evaluate any proposed crate additions carefully for compile time, security footprint, and runtime overhead.

---

## Pull Request Process

1. Create a dedicated topic branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Implement your changes with clear, semantic commit messages.
3. Ensure `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` pass.
4. Open a pull request against the `main` branch using the provided [Pull Request Template](.github/pull_request_template.md).
5. Address any review feedback promptly.

---

## Reporting Issues

- For reproducible bugs, please submit a report using the [Bug Report Template](.github/ISSUE_TEMPLATE/bug_report.md).
- For feature requests or proposals, use the [Feature Request Template](.github/ISSUE_TEMPLATE/feature_request.md).
