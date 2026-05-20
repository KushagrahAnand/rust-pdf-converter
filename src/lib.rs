use std::{path::PathBuf, process};

use printpdf::*;

const PAGE_WIDTH_MM:f32 = 210.0;
const PAGE_HEIGHT_MM:f32 = 297.0;
const PAGE_HEIGHT_PT:f32 = 842.0;
const TOP_MARGIN_OFFSET:f32 = 50.0;
const BOTTOM_MARGIN:f32 = 50.0;
const LEFT_MARGIN:f32 = 50.0;
const RIGHT_MARGIN:f32 = 495.5;

pub fn create_pdf(data: String, font_path: PathBuf, size: f32) -> Vec<u8> {
    let mut doc = PdfDocument::new("PDF");

    let font = get_font(font_path);
    let font_id = doc.add_font(&font);

    let size = Pt(size);
    let line_height = Pt(size.0 * 1.25);

    let page_height = Pt(PAGE_HEIGHT_PT);
    let top_margin = page_height - Pt(TOP_MARGIN_OFFSET);
    let bottom_margin = Pt(BOTTOM_MARGIN);
    let left_margin = Pt(LEFT_MARGIN);
    let right_margin = Pt(RIGHT_MARGIN);
    let max_width = (right_margin - left_margin).0;

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
            flush_page(&mut pages, &mut current_page_ops, &mut current_y, top_margin, new_page_ops());
        }

        get_line(font.clone(), size, line, max_width, &mut current_page_ops, &mut current_y, line_height, bottom_margin, &mut pages, top_margin, new_page_ops());
    }

    if current_y < top_margin {
        let page = PdfPage::new(Mm(210.0), Mm(297.0), current_page_ops);
        pages.push(page);
    }

    let mut warnings = Vec::new();
    doc.with_pages(pages).save(&PdfSaveOptions::default(), &mut warnings)
}


fn get_font(font_path: PathBuf) -> ParsedFont{
    let font_bytes = match std::fs::read(font_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error in getting font file: {}", e);
            process::exit(1);
        }
    };
    let font_index = 0;
    let mut warnings = Vec::new();
    match ParsedFont::from_bytes(&font_bytes, font_index, &mut warnings) {
        Some(font) => font,
        None => {
            eprintln!("Error in font file");
            process::exit(1);
        },
    }
}


fn build_page_ops(
    font_id:FontId, 
    size: Pt, 
    line_height: Pt, 
    left_margin:Pt, 
    top_margin:Pt 
) -> Vec<Op> {
    vec![
        Op::SetFont {
            font: PdfFontHandle::External(font_id.clone()),
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
    pages:&mut Vec<PdfPage>, 
    current_page_ops:&mut Vec<Op>, 
    current_y:&mut Pt, 
    top_margin:Pt, 
    new_page_ops:Vec<Op>
) {
    let page = PdfPage::new(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), current_page_ops.clone());
    pages.push(page);
    *current_page_ops = new_page_ops;
    *current_y = top_margin;
}


fn get_line(
    font:ParsedFont,
    size: Pt,
    line:&str,
    max_width:f32,
    current_page_ops:&mut Vec<Op>,
    current_y:&mut Pt,
    line_height: Pt,
    bottom_margin: Pt,
    pages:&mut Vec<PdfPage>,
    top_margin:Pt,
    new_page_ops:Vec<Op>
) {
    let space_width = measure_word(" ", &font, size);
        let mut current_row = String::new();
        let mut current_row_width = Pt(0.0);

        for word in line.split_whitespace() {
            let word_width = measure_word(word, &font, size);

            if (current_row_width + word_width + space_width).0 > max_width {
                if !current_row.is_empty() {
                    current_page_ops.push(Op::ShowText {
                        items: vec![TextItem::Text(current_row.clone())],
                    });
                    current_page_ops.push(Op::AddLineBreak);
                    *current_y -= line_height;

                    if *current_y < bottom_margin {
                        let page = PdfPage::new(Mm(210.0), Mm(297.0), current_page_ops.clone());
                        pages.push(page);
                        *current_page_ops = new_page_ops.clone();
                        *current_y = top_margin;
                    }
                }

                current_row = word.to_string();
                current_row_width = word_width;
            } else {
                if !current_row.is_empty() {
                    current_row.push(' ');
                    current_row_width = current_row_width + space_width;
                }
                current_row.push_str(word);
                current_row_width = current_row_width + word_width;
            }
        }

        if !current_row.is_empty() {
            current_page_ops.push(Op::ShowText {
                items: vec![TextItem::Text(current_row)],
            });
            current_page_ops.push(Op::AddLineBreak);
            *current_y -= line_height;
        }
}


fn measure_word(word: &str, font: &ParsedFont, size: Pt) -> Pt {
    let units_per_em = font.font_metrics.units_per_em as f32;
    let mut word_width = 0.0;

    for ch in word.chars() {
        let glyph_index = match font.lookup_glyph_index(ch as u32) {
            Some(num) => num,
            None => continue,
        };
        if let Some(advance) = font.get_glyph_width_internal(glyph_index) {
            word_width += (advance as f32 / units_per_em) * size.0;
        }
    }

    Pt(word_width)
}