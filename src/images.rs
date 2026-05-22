use printpdf::*;
use ::image;
use std::{path::PathBuf, process};

pub fn create_pdf_img(path:PathBuf) -> Vec<u8> {
    let mut doc = PdfDocument::new("pdf");
    let mut pages = Vec::new();

    let dyn_image = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error in loading image file : {}", e);
            process::exit(1);
        }
    };

    let img_width_px = dyn_image.width() as f32;
    let img_height_px = dyn_image.height() as f32;

    let raw_image = match RawImage::from_dynamic_image(dyn_image) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error : {}", e);
            process::exit(1);
        }
    };

    let dpi = 96.0_f32;
    let img_width_mm = (img_width_px / dpi) * 25.4;
    let img_height_mm = (img_height_px / dpi) * 25.4;

    let max_w = 190.0_f32;
    let max_h = 277.0_f32;
    let scale = (max_w / img_width_mm).min(max_h / img_height_mm);
    let final_w = img_width_mm * scale;
    let final_h = img_height_mm * scale;

    let x = (210.0_f32 - final_w) / 2.0;
    let y = (297.0_f32 - final_h) / 2.0;

    let image_xobject_id = doc.add_image(&raw_image);

    let page_ops = vec![
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

    let page = PdfPage::new(Mm(210.0), Mm(297.0), page_ops);
    pages.push(page);

    let mut warnings = Vec::new();
    doc.with_pages(pages).save(&PdfSaveOptions::default(), &mut warnings)
}