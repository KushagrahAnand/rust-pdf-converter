use std::fs::{File, write};
use std::io::Read;
use std::path::PathBuf;
use std::process;
use converter::{create_pdf_text, create_pdf_docx};
use converter::images::create_pdf_img;
use converter::docx::read_docx;
use clap::Parser;

#[derive(Parser)]
///Tool to create pdf from text file
#[command(version, about, long_about = None)]
struct Cli {
    ///Name/Path of text file
    #[arg(short, long)]
    input: PathBuf,

    ///Name/Path of output pdf
    #[arg(short, long)]
    output: PathBuf,

    ///Font. Some fonts are present inside ./assets
    #[arg(short, long, default_value="./assets/fonts/roboto/Roboto-Regular.ttf")]
    font: PathBuf,

    ///Text size
    #[arg(short, long, default_value_t = 16.0)]
    size: f32,

    ///Page size (a4, a3, letter, legal)
    #[arg(short, long, default_value="a4")]
    paper: String,

    ///Orientation (portrait, landscape)
    #[arg(short = 'r', long, default_value="portrait")]
    orientation: String,
}

fn main() {
    let args = Cli::parse();

    let extension = args.input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let pdf_bytes = match extension.as_str() {
        "jpg" | "jpeg" | "png" | "webp" | "bmp" | "tiff" | "gif" | "img" => {
            create_pdf_img(args.input)
        },
        "txt" => {
            let mut file = match File::open(args.input) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error in getting input text file: {}", e);
                    process::exit(1);
                },
            };

            let mut data = String::new();
            if let Err(e) = file.read_to_string(&mut data) {
                eprintln!("Error: {}", e);
                process::exit(1);
            };

            create_pdf_text(data, args.font, args.size, args.paper, args.orientation)
        },
        "docx" => {
            let blocks = read_docx(args.input);
            create_pdf_docx(blocks, args.font, args.size, args.paper, args.orientation)
        },
        _ => {
            eprintln!("Unsupported file format: {}. Supported: txt, docx, jpg, jpeg, png, webp, bmp, tiff, gif, img", extension);
            process::exit(1);
        }
    };

    save_pdf(pdf_bytes, args.output);
}

fn save_pdf(pdf_bytes: Vec<u8>, output: PathBuf) {
    match write(output, pdf_bytes) {
        Ok(_) => println!("Successfully created pdf"),
        Err(e) => {
            eprintln!("Error in creating pdf: {}", e);
            process::exit(1);
        }
    }
}