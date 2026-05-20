# rust-pdf-converter

A fast, lightweight Rust CLI tool that converts text files into cleanly structured, multi-page PDF documents using the `printpdf` and `clap` crates.

## Features

- **CLI Interface:** Fully functional command line interface built with `clap`.
- **Custom Typography:** Supports custom TrueType fonts passed dynamically via CLI. Several fonts included in `./assets/fonts`.
- **Dynamic Multi-Page Engine:** Automatically paginates text across multiple pages without layout fractures or cut-offs.
- **Word Wrapping:** Automatically wraps long lines using real font glyph width measurement for accurate line breaking.
- **Configurable:** Font size, font path, input and output paths all configurable via flags.

## Usage

```bash
converter --input <PATH> --output <PATH> [--font <PATH>] [--size <POINTS>]
```

### Arguments

| Flag | Short | Description | Default |
|---|---|---|---|
| `--input` | `-i` | Path to the source `.txt` file | required |
| `--output` | `-o` | Path for the generated PDF | required |
| `--font` | `-f` | Path to a `.ttf` font file | `./assets/fonts/Roboto-Regular.ttf` |
| `--size` | `-s` | Font size in points | `16.0` |

### Example

```bash
converter -i ./assets/samples/sample.txt -o output.pdf -f ./assets/fonts/Roboto-Regular.ttf -s 16
```

## Installation

1. Clone the repository and navigate to the project root:
```bash
git clone https://github.com/KushagrahAnand/rust-pdf-converter.git
cd rust-pdf-converter
```

2. Build the project:
```bash
cargo build --release
```

3. Run it:
```bash
# Windows
.\target\release\converter --input input.txt --output output.pdf

# Linux/Mac
./target/release/converter --input input.txt --output output.pdf
```

## Assets

The repository includes ready-to-use assets:

- **`./assets/fonts/`** — bundled TrueType fonts to use out of the box
- **`./assets/samples/`** — sample `.txt` files to test the converter

## Roadmap

- [x] Core text-to-pdf conversion
- [x] Dynamic multi-page pagination
- [x] CLI interface with `clap`
- [x] Word wrapping with glyph-accurate line measurement
- [x] Modular code structure
- [ ] Custom margin and line-height flags
- [ ] `.docx` to PDF conversion
- [ ] `.pptx` to PDF conversion