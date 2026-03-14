# Lightweight Markdown Viewer (MdViewer)

A high-performance, memory-efficient Markdown viewer for Windows, Linux, and macOS. Built with Rust and the [egui](https://github.com/emilk/egui) framework.

## Features

- **Lightweight**: Minimal CPU and memory usage (native hardware-accelerated GUI).
- **Fast**: Near-instant startup and smooth scrolling.
- **Cross-Platform**: Compiles to native binaries for major desktop platforms.
- **CLI Support**: Open files directly from the terminal.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Build from Source

```bash
git clone https://github.com/AlexJ-StL/myMarkDownViewer.git
cd myMarkDownViewer
cargo build --release
```

To install the viewer globally so you can use the `md_viewer` command from anywhere on your system:
```bash
cargo install --path .
```

If you don't install it globally, the optimized binary is located locally at `target/release/md_viewer.exe`.

## Usage

### GUI
Run the application and click **📂 Open File...** to select a Markdown file.

### CLI
Open a file directly from the terminal:
```bash
md_viewer path/to/your/file.md
```

## Security & Integrity
This application is built with memory safety in mind using Rust. It only requires read access to the files you choose to open. No network access is required or utilized.
