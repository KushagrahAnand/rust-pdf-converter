mod words;
mod page_layout;

use std::{path::PathBuf, process};
use printpdf::*;
use crate::words::*;
use crate::page_layout::*;

pub fn create_pdf(data: String, font_path: PathBuf, size: f32, paper: String, orientation: String) -> Vec<u8> {
    let mut doc = PdfDocument::new("PDF");

    let font = get_font(font_path);
    let font_id = doc.add_font(&font);

    let size = Pt(size);
    let line_height = Pt(size.0 * 1.25);

    let layout = match paper.parse::<PSize>() {
        Ok(p) => p.dimensions(),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let layout = if orientation.to_lowercase() == "landscape" {
        layout.landscape()
    } else {
        layout
    };

    let top_margin = Pt(layout.top_margin);
    let bottom_margin = Pt(layout.bottom_margin);
    let left_margin = Pt(layout.left_margin);
    let right_margin = Pt(layout.right_margin);
    let max_width = (right_margin - left_margin).0;
    let width = layout.width;
    let height = layout.height;

    let new_page_ops = move || build_page_ops(font_id.clone(), size, line_height, left_margin, top_margin);

    let mut pages = Vec::new();
    let mut current_page_ops = new_page_ops();
    let mut current_y = top_margin;

    for line in data.lines() {
        if line.trim().is_empty() {
            current_y -= line_height;
            current_page_ops.push(Op::AddLineBreak);
            continue;
        }

        if current_y < bottom_margin {
            flush_page(&mut pages, &mut current_page_ops, &mut current_y, top_margin, new_page_ops(), width, height);
        }

        get_line(font.clone(), &mut pages, size, line, max_width, &mut current_page_ops, &mut current_y, line_height, bottom_margin, top_margin, new_page_ops(), width, height);
    }

    if current_y < top_margin {
        let page = PdfPage::new(width, height, current_page_ops);
        pages.push(page);
    }

    let mut warnings = Vec::new();
    doc.with_pages(pages).save(&PdfSaveOptions::default(), &mut warnings)
}

fn get_font(font_path: PathBuf) -> ParsedFont {
    let font_bytes = match std::fs::read(font_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error in getting font file: {}", e);
            process::exit(1);
        }
    };
    let mut warnings = Vec::new();
    match ParsedFont::from_bytes(&font_bytes, 0, &mut warnings) {
        Some(font) => font,
        None => {
            eprintln!("Error in font file");
            process::exit(1);
        }
    }
}

fn build_page_ops(
    font_id: FontId,
    size: Pt,
    line_height: Pt,
    left_margin: Pt,
    top_margin: Pt,
) -> Vec<Op> {
    vec![
        Op::SetFont {
            font: PdfFontHandle::External(font_id),
            size,
        },
        Op::SetLineHeight { lh: line_height },
        Op::SetTextCursor {
            pos: Point {
                x: left_margin,
                y: top_margin,
            },
        },
    ]
}

fn flush_page(
    pages: &mut Vec<PdfPage>,
    current_page_ops: &mut Vec<Op>,
    current_y: &mut Pt,
    top_margin: Pt,
    new_page_ops: Vec<Op>,
    width: Mm,
    height: Mm,
) {
    let page = PdfPage::new(width, height, current_page_ops.clone());
    pages.push(page);
    *current_page_ops = new_page_ops;
    *current_y = top_margin;
}