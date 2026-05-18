use printpdf::*;

pub fn create_pdf(data: String) -> Vec<u8> {
    let mut doc = PdfDocument::new("Output");

    let font_bytes = include_bytes!("../assets/fonts/Roboto-Regular.ttf");
    let font_index = 0;
    let mut warnings = Vec::new();
    let font = ParsedFont::from_bytes(font_bytes, font_index, &mut warnings).unwrap();

    let font_id = doc.add_font(&font);
    let size: Pt = Pt(16.0);
    let line_height = Pt(20.0);

    let top_margin = Pt(750.0);
    let bottom_margin = Pt(50.0);
    let left_margin = Pt(50.0);

    let mut pages = Vec::new();

    let new_page_ops = move || vec![
        Op::SetFont {
            font: PdfFontHandle::External(font_id.clone()),
            size,
        },
        Op::SetCharacterSpacing { multiplier: 2.0 },
        Op::SetLineHeight { lh: line_height },
        Op::SetTextCursor {
            pos: Point {
                x: left_margin,
                y: top_margin,
            },
        },
    ];

    let mut current_page_ops = new_page_ops();
    let mut current_y = top_margin;

    for line in data.lines() {
        if line.trim().is_empty() {
            current_y -= line_height;
            current_page_ops.push(Op::AddLineBreak);
            continue;
        }

        if current_y < bottom_margin {
            let page = PdfPage::new(Mm(210.0), Mm(297.0), current_page_ops);
            pages.push(page);

            current_page_ops = new_page_ops();
            current_y = top_margin;
        }

        current_page_ops.push(Op::ShowText {
            items: vec![TextItem::Text(line.to_string())],
        });
        current_page_ops.push(Op::AddLineBreak);

        current_y -= line_height;
    }

    if current_y < top_margin {
        let page = PdfPage::new(Mm(210.0), Mm(297.0), current_page_ops);
        pages.push(page);
    }

    let mut warnings = Vec::new();
    doc.with_pages(pages).save(&PdfSaveOptions::default(), &mut warnings)
}