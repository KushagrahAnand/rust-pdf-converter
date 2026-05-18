use printpdf::*;

pub fn create_pdf(data:String) -> Vec<u8>{
    let mut doc = PdfDocument::new("Output");

    let font_bytes = include_bytes!("../assests/fonts/Roboto-Regular.ttf");
    let font_index = 0;
    let mut warnings = Vec::new();
    let font = ParsedFont::from_bytes(font_bytes, font_index, &mut warnings).unwrap();

    let font_id = doc.add_font(&font);
    let font_handle = PdfFontHandle::External(font_id);
    let size:Pt = Pt(16.0);
    let line_height = Pt(20.0);

    let mut pages = Vec::new();

    let mut current_page_ops = vec![
        Op::SetFont { font: font_handle, size: size},
        Op::SetCharacterSpacing { multiplier: 2.0 },
        Op::SetLineHeight { lh: line_height },
    ];
    
    let left_margin = Pt(50.0);
    let top_margin = Pt(750.0);
    let bottom_margin = Pt(50.0);

    let mut current_y = top_margin;

    for line in data.lines() {
        if line.trim().is_empty() {
            current_y -= line_height;
            continue;
        };
        if current_y < bottom_margin {
            current_y = top_margin;

            let page = PdfPage::new(Mm(210.0), Mm(297.0), current_page_ops.clone());
            pages.push(page);
        };

        let line_ops = vec![
            Op::SetTextCursor { 
                pos: Point { 
                    x: left_margin, 
                    y: current_y 
                } 
            },
            Op::ShowText { 
                items: vec![TextItem::Text(line.to_string())],
            },
        ];

        current_page_ops.extend(line_ops);

        current_y -= line_height;

    };

    if !current_page_ops.is_empty() {
        let page = PdfPage::new(Mm(210.0), Mm(297.0), current_page_ops.clone());
        pages.push(page);
    }

    let mut warnings = Vec::new();
    doc.with_pages(pages).save(&PdfSaveOptions::default(), &mut warnings)
}