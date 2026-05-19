# rust-pdf-converter

A fast, lightweight Rust CLI tool that converts text files into cleanly structured, multi-page PDF documents using the `printpdf` and `clap` crates.

## Features

- **CLI Interface:** Fully functional command line interface built with `clap`.
- **Custom Typography:** Supports custom TrueType fonts passed dynamically via CLI.
- **Dynamic Multi-Page Engine:** Automatically paginates text across multiple pages without layout fractures or cut-offs.
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
   ./target/release/converter --input input.txt --output output.pdf
```

## Roadmap

- [x] Core text-to-pdf conversion
- [x] Dynamic multi-page pagination
- [x] CLI interface with `clap`
- [ ] Word wrapping for long lines
- [ ] Custom margin and line-height flags
- [ ] `.docx` to PDF conversion
- [ ] `.pptx` to PDF conversion