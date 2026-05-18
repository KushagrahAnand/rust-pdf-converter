# rust-pdf-converter

A lightweight Rust application designed to read text payloads and compile them into structured PDF documents using the `printpdf` crate.

## 🚀 Current Progress & Features

As of today, the core pipeline infrastructure is fully set up and connected between the library and binary modules:

* **File I/O Stream:** The application successfully locates, opens, and reads raw text data from a local `index.txt` file.
* **PDF Core Initialization:** Fully configures the `printpdf` document context, allocates canvas dimensions matching standard A4 paper sizes, and manages custom typography embedding via `ParsedFont` (using `Roboto-Regular.ttf`).
* **Text Layout Engine:** Successfully sets up absolute coordinate transforms (`Op::SetTextCursor`) and pushes glyph vectors (`Op::ShowText`) to the layout stream.
* **Milestone Reached:** The engine successfully captures and cleanly prints the **very first line** of the input `.txt` file onto the final generated `output.pdf`.

---

## 🛠️ Known Issues & Current Limitations

While the foundational pipeline works, the layout engine currently encounters structural boundaries when processing continuous text bodies:

1.  **Missing Horizontal Text Wrapping:** `printpdf` does not natively wrap text string buffers. Long paragraphs currently print strictly along the horizontal X-axis, trailing off the right side of the physical page canvas.
2.  **Vertical Layout Trapping:** The coordinate pointer does not automatically transition to a new line following an execution block. Currently, the text rendering drops off or forces a page fracture after the initial line execution.

---

## 🗺️ Next Milestones / To-Do List

To turn this into a fully functional document generation tool, the immediate next focus areas are:

- [ ] Implement a custom word-wrapping utility to mathematically break long strings into lines based on maximum horizontal page boundaries.
- [ ] Fix the vertical coordinate tracking (`current_y`) state engine to allow continuous line increments down a single page.
- [ ] Implement multi-page safety triggers when text lines surpass the bottom margin threshold.
- [ ] Expand file parsing capabilities to lay the groundwork for extracting data blocks from `.docx` and `.pptx` architectures.

---

## 📦 Local Installation & Usage

To build and test the current layout pipeline locally:

1. Clone the repository and navigate to the project root:
   ```bash
   git clone https://github.com/KushagrahAnand/rust-pdf-converter.git
   cd rust-pdf-converter
