# rust-pdf-converter

A fast, lightweight Rust CLI tool that converts text files and images into cleanly structured PDF documents using the `printpdf` and `clap` crates.

## Features

- **CLI Interface:** Fully functional command line interface built with `clap`.
- **Custom Typography:** Supports custom TrueType fonts passed dynamically via CLI. Several fonts included in `./assets/fonts`.
- **Dynamic Multi-Page Engine:** Automatically paginates text across multiple pages without layout fractures or cut-offs.
- **Word Wrapping:** Automatically wraps long lines using real font glyph width measurement for accurate line breaking.
- **Image to PDF:** Converts a wide range of image formats directly to PDF.
- **Configurable:** Font size, font path, page dimensions, orientation, input, and output paths are all configurable via flags.

## Supported Input Formats

| Format | Extension |
|---|---|
| Text | `.txt` |
| JPEG | `.jpg`, `.jpeg` |
| PNG | `.png` |
| WebP | `.webp` |
| BMP | `.bmp` |
| TIFF | `.tiff` |
| GIF | `.gif` |
| Image | `.img` |

## Usage

```bash
converter --input <PATH> --output <PATH> [--font <PATH>] [--size <POINTS>] [--paper <SIZE>] [--orientation <ORIENTATION>]
```

For image conversion, only `--input` and `--output` are required. All other flags are ignored.

```bash
converter --input image.jpg --output output.pdf
```

### Arguments

| Flag | Short | Description | Default |
|---|---|---|---|
| `--input` | `-i` | Path to the source file | required |
| `--output` | `-o` | Path for the generated PDF | required |
| `--font` | `-f` | Path to a `.ttf` font file (text only) | `./assets/fonts/Roboto-Regular.ttf` |
| `--size` | `-s` | Font size in points (text only) | `16.0` |
| `--paper` | `-p` | Paper size: `a4`, `a3`, `letter`, `legal` (text only) | `a4` |
| `--orientation` | `-r` | Orientation: `portrait`, `landscape` (text only) | `portrait` |

### Examples

```bash
# Text to PDF
converter -i ./assets/samples/sample.txt -o output.pdf -p a4 -r landscape -s 14.0

# Image to PDF
converter -i photo.jpg -o output.pdf
converter -i photo.png -o output.pdf
converter -i photo.webp -o output.pdf
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
- **`./assets/imgsamples/`** — sample images to test image to PDF conversion

## Roadmap

- [x] Core text-to-pdf conversion
- [x] Dynamic multi-page pagination
- [x] CLI interface with `clap`
- [x] Word wrapping with glyph-accurate line measurement
- [x] Modular code structure
- [x] Support for diverse page sizes and layout orientations
- [x] Image to PDF conversion (`.jpg`, `.jpeg`, `.png`, `.webp`, `.bmp`, `.tiff`, `.gif`, `.img`)
- [ ] `.docx` to PDF conversion
- [ ] `.pptx` to PDF conversion