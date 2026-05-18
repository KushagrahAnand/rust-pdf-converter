# rust-pdf-converter

A fast, lightweight Rust application designed to parse text payloads and compile them into cleanly structured, multi-page PDF documents using the `printpdf` crate.

## 🚀 Current Progress & Features

The core document generation pipeline is fully operational:

* **File I/O Stream:** Automatically locates, reads, and processes raw text streams from local files.
* **Custom Typography:** Standardizes text styling using embedded external vector assets (`Roboto-Regular.ttf`) mapped via `ParsedFont`.
* **Dynamic Multi-Page Engine:** Features a fully resolved pagination and layout rendering engine. The application successfully wraps, structures, and processes complex, multi-line paragraphs, dynamically flowing text continuously across multiple pages without layout fractures or cut-offs.

---

## 🎯 Next Target: Transforming into a CLI Tool

To make this converter highly modular and production-ready, the immediate development focus is transitioning the binary from a hardcoded script into a flexible Command Line Interface (CLI). 

### Planned CLI Arguments & Flags:
* `-i, --input <PATH>`: Specify the source file to convert (e.g., `.txt`, and eventually `.docx`/`.pptx`).
* `-o, --output <PATH>`: Customize the destination file name and path for the generated PDF.
* `-f, --font <PATH>`: Allow users to pass a custom TrueType (`.ttf`) font file dynamically.
* `-s, --size <POINTS>`: Set the font size directly from the terminal (defaults to `16.0`).

---

## 🗺️ Long-Term Roadmap

- [x] Resolve word-wrapping and multi-page text continuation gaps.
- [ ] Implement a command-line interface argument parser (using `clap`).
- [ ] Add support for custom layout styling flags (margins, line-height overrides).
- [ ] Extend file parsing modules to extract structural content blocks from `.docx` and `.pptx` files.

---

## 📦 Local Installation & Usage

To build and run the current multi-page compilation pipeline locally:

1. Clone the repository:
   ```bash
   git clone https://github.com/KushagrahAnand/rust-pdf-converter.git
   cd rust-pdf-converter