use printpdf::*;

pub fn get_line(
    font:ParsedFont,
    pages:&mut Vec<PdfPage>,
    size: Pt,
    line:&str,
    max_width:f32,
    current_page_ops:&mut Vec<Op>,
    current_y:&mut Pt,
    line_height: Pt,
    bottom_margin: Pt,
    
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


pub fn measure_word(word: &str, font: &ParsedFont, size: Pt) -> Pt {
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