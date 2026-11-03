---
name: Bug Report
about: Report a reproducible bug or unexpected behavior in glyph
title: "[BUG] "
labels: bug
assignees: ""
---

## Describe the Bug

A clear and concise description of the issue encountered.

## Steps to Reproduce

1. Run command: `glyph [options]`
2. Select region on screen (or describe the action taken)
3. Observe error or unexpected behavior

## Expected Behavior

A clear description of what you expected to happen.

## Actual Behavior

What actually happened. Include full terminal output or error messages:

```text
# Terminal output / RUST_LOG=debug glyph
```

## Environment

- **OS / Distribution**: [e.g. Arch Linux, Fedora 40, Ubuntu 24.04]
- **Wayland Compositor**: [e.g. Hyprland 0.45, Sway 1.9, River, GNOME Wayland]
- **glyph Version**: [e.g. 0.1.0 — run `glyph --version`]
- **Slurp Version**: [e.g. run `slurp -v`]
- **Grim Version**: [e.g. run `grim -v`]
- **Tesseract Version**: [e.g. run `tesseract --version`]
- **Installed via**: [AUR (glyph-bin/glyph-git) / Cargo / Source]

## Additional Context

Add any other context, such as display scaling factor, multi-monitor configuration, or screenshots.
