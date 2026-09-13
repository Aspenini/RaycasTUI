# Raycast TUI

A cross-platform terminal raycaster written in Rust 2024. Classic DDA perspective in any ANSI 256-color terminal.

## Features

- **Cross-platform**: Windows, Linux, and macOS via crossterm
- **Half-block rendering**: two vertical pixels per character cell
- **Held-key movement**: frame-rate independent, with wall sliding
- **Safe shutdown**: the terminal is restored on quit, error, or panic

## Controls

- **W / ↑**: Move forward
- **S / ↓**: Move backward
- **A**: Strafe left
- **D**: Strafe right
- **←**: Rotate left
- **→**: Rotate right
- **Q / Esc / Ctrl+C**: Quit

## Building

Rust 1.85 or newer is required.

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

Or run the release binary directly:

```bash
./target/release/raycast-tui
```

On Windows that is `target\release\raycast-tui.exe`.

## How It Works

Rays are cast with a Digital Differential Analyzer (DDA) from the player's camera plane. Each ray's perpendicular wall distance sets the column height; east/west vs north/south hits are shaded differently for depth. The map is a compile-time 24×24 grid (`#` is a wall). Movement uses delta time so speed stays consistent if a frame hiccups.

The renderer writes a single ANSI frame per tick using `▀` half-blocks, so each terminal cell shows two stacked 256-color pixels.

## Requirements

- Rust 1.85+ (edition 2024)
- A terminal that supports ANSI 256-color output

## License

This project is open source and available for use.
