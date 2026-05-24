# rust-pdf-converter

A fast, lightweight Rust CLI tool that converts text files, Word documents, and images into cleanly structured PDF documents using the `printpdf` and `clap` crates.

## Features

- **CLI Interface:** Fully functional command line interface built with `clap`.
- **Custom Typography:** Supports custom TrueType fonts passed dynamically via CLI. Several fonts included in `./assets/fonts`.
- **Dynamic Multi-Page Engine:** Automatically paginates text across multiple pages without layout fractures or cut-offs.
- **Word Wrapping:** Automatically wraps long lines using real font glyph width measurement for accurate line breaking.
- **Image to PDF:** Converts a wide range of image formats directly to PDF.
- **Word Document to PDF:** Converts `.docx` files to PDF, extracting paragraphs, tables, images, and page breaks.
- **Multi-File Merging:** Merge multiple files of the same type into a single PDF, with each file starting on a new page.
- **Configurable:** Font size, font path, page dimensions, orientation, input, and output paths are all configurable via flags.

## Supported Input Formats

| Format | Extension |
|---|---|
| Text | `.txt` |
| Word Document | `.docx` |
| JPEG | `.jpg`, `.jpeg` |
| PNG | `.png` |
| WebP | `.webp` |
| BMP | `.bmp` |
| TIFF | `.tiff` |
| GIF | `.gif` |
| Image | `.img` |

## Usage

```bash
converter --input <PATH> [<PATH>...] --output <PATH> [--font <PATH>] [--size <POINTS>] [--paper <SIZE>] [--orientation <ORIENTATION>]
```

For image conversion, only `--input` and `--output` are required. All other flags are ignored.

```bash
converter --input image.jpg --output output.pdf
```

### Arguments

| Flag | Short | Description | Default |
|---|---|---|---|
| `--input` | `-i` | One or more input files (must be same extension) | required |
| `--output` | `-o` | Path for the generated PDF | required |
| `--font` | `-f` | Path to a `.ttf` font file (text/docx only) | `./assets/fonts/Roboto-Regular.ttf` |
| `--size` | `-s` | Font size in points (text/docx only) | `16.0` |
| `--paper` | `-p` | Paper size: `a4`, `a3`, `letter`, `legal` (text/docx only) | `a4` |
| `--orientation` | `-r` | Orientation: `portrait`, `landscape` (text/docx only) | `portrait` |

### Examples

```bash
# Single text file to PDF
converter -i ./assets/samples/sample.txt -o output.pdf -p a4 -r landscape -s 14.0

# Merge multiple text files into one PDF
converter -i file1.txt file2.txt file3.txt -o combined.pdf

# Single Word document to PDF
converter -i document.docx -o output.pdf

# Merge multiple Word documents into one PDF
converter -i doc1.docx doc2.docx doc3.docx -o combined.pdf

# Single image to PDF
converter -i photo.jpg -o output.pdf

# Merge multiple images into one PDF
converter -i photo1.jpg photo2.jpg photo3.jpg -o combined.pdf
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
- [x] `.docx` to PDF conversion (paragraphs, tables, images, page breaks)
- [x] Multi-file merging into a single PDF
- [ ] `.pptx` to PDF conversion