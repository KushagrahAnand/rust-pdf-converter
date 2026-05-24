mod words;
mod page_layout;
pub mod images;
pub mod docx;

use std::{path::PathBuf, process};
use printpdf::*;
use ::image::load_from_memory;
use crate::words::*;
use crate::page_layout::*;
use crate::docx::DocxBlock;

pub fn create_pdf_text(data: String, font_path: PathBuf, size: f32, paper: String, orientation: String) -> Vec<u8> {
    let mut doc = PdfDocument::new("PDF");

    let font = get_font(font_path);
    let font_id = doc.add_font(&font);

    let size = Pt(size);
    let line_height = Pt(size.0 * 1.25);

    let (top_margin, bottom_margin, left_margin, right_margin, width, height) = get_layout(paper, orientation);
    let max_width = (right_margin - left_margin).0;

    let new_page_ops = move || build_page_ops(font_id.clone(), size, line_height, left_margin, top_margin);

    let mut pages = Vec::new();
    let mut current_page_ops = new_page_ops();
    let mut current_y = top_margin;

    for line in data.lines() {
        if line.contains('\x0C') {
            flush_page(&mut pages, &mut current_page_ops, &mut current_y, top_margin, new_page_ops(), width, height);
            continue;
        }
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

pub fn create_pdf_docx(blocks: Vec<DocxBlock>, font_path: PathBuf, size: f32, paper: String, orientation: String) -> Vec<u8> {
    let mut doc = PdfDocument::new("PDF");

    let font = get_font(font_path);
    let font_id = doc.add_font(&font);

    let size = Pt(size);
    let line_height = Pt(size.0 * 1.25);

    let (top_margin, bottom_margin, left_margin, right_margin, width, height) = get_layout(paper, orientation);
    let max_width = (right_margin - left_margin).0;

    let new_page_ops = move || build_page_ops(font_id.clone(), size, line_height, left_margin, top_margin);

    let mut pages = Vec::new();
    let mut current_page_ops = new_page_ops();
    let mut current_y = top_margin;

    for block in blocks {
        match block {
            DocxBlock::Text(text) => {
                for line in text.lines() {
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
            },
            DocxBlock::PageBreak => {
                flush_page(&mut pages, &mut current_page_ops, &mut current_y, top_margin, new_page_ops(), width, height);
            },
            DocxBlock::Image(bytes) => {
                if current_y < top_margin {
                    flush_page(&mut pages, &mut current_page_ops, &mut current_y, top_margin, new_page_ops(), width, height);
                }

                let dyn_image = match load_from_memory(&bytes) {
                    Ok(img) => img,
                    Err(e) => {
                        eprintln!("Error loading image from docx: {}", e);
                        continue;
                    }
                };

                let img_width_px = dyn_image.width() as f32;
                let img_height_px = dyn_image.height() as f32;

                let raw_image = match RawImage::from_dynamic_image(dyn_image) {
                    Ok(img) => img,
                    Err(e) => {
                        eprintln!("Error processing image: {}", e);
                        continue;
                    }
                };

                let dpi = 96.0_f32;
                let img_width_mm = (img_width_px / dpi) * 25.4;
                let img_height_mm = (img_height_px / dpi) * 25.4;

                let max_w = width.0 as f32 - 20.0;
                let max_h = height.0 as f32 - 20.0;
                let scale = (max_w / img_width_mm).min(max_h / img_height_mm);
                let final_w = img_width_mm * scale;
                let final_h = img_height_mm * scale;

                let x = (width.0 as f32 - final_w) / 2.0;
                let y = (height.0 as f32 - final_h) / 2.0;

                let image_xobject_id = doc.add_image(&raw_image);

                let img_page_ops = vec![
                    Op::UseXobject {
                        id: image_xobject_id,
                        transform: XObjectTransform {
                            translate_x: Some(Mm(x).into()),
                            translate_y: Some(Mm(y).into()),
                            scale_x: Some(scale),
                            scale_y: Some(scale),
                            dpi: Some(dpi),
                            ..Default::default()
                        }
                    },
                ];

                let page = PdfPage::new(width, height, img_page_ops);
                pages.push(page);

                current_page_ops = new_page_ops();
                current_y = top_margin;
            },
        }
    }

    if current_y < top_margin {
        let page = PdfPage::new(width, height, current_page_ops);
        pages.push(page);
    }

    let mut warnings = Vec::new();
    doc.with_pages(pages).save(&PdfSaveOptions::default(), &mut warnings)
}

// ── Private helpers ──────────────────────────────────────────────────────────

fn get_layout(paper: String, orientation: String) -> (Pt, Pt, Pt, Pt, Mm, Mm) {
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

    (
        Pt(layout.top_margin),
        Pt(layout.bottom_margin),
        Pt(layout.left_margin),
        Pt(layout.right_margin),
        layout.width,
        layout.height,
    )
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