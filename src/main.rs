use std::fs::{File, write};
use std::io::Read;
use std::path::PathBuf;
use std::process;
use converter::{create_pdf_text, create_pdf_docx};
use converter::images::create_pdf_img;
use converter::docx::{DocxBlock, read_docx};
use clap::Parser;

#[derive(Parser)]
///Tool to create pdf from text file
#[command(version, about, long_about = None)]
struct Cli {
    /// Input files (all must be same extension)
    #[arg(short, long, num_args = 1..)]
    input: Vec<PathBuf>,

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

    // verify all files have the same extension
    let extensions: Vec<&str> = args.input
        .iter()
        .map(|p| p.extension().and_then(|e| e.to_str()).unwrap_or(""))
        .collect();

    if extensions.windows(2).any(|w| w[0] != w[1]) {
        eprintln!("Error: all input files must have the same extension");
        process::exit(1);
    }

    let extension = extensions[0].to_lowercase();

    let pdf_bytes = match extension.as_str() {
        "jpg" | "jpeg" | "png" | "webp" | "bmp" | "tiff" | "gif" | "img" => {
            if args.input.len() == 1 {
                create_pdf_img(args.input.into_iter().next().unwrap())
            } else {
                let mut all_blocks: Vec<DocxBlock> = Vec::new();
                for path in args.input {
                    let bytes = match std::fs::read(&path) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Error reading image {:?}: {}", path, e);
                            process::exit(1);
                        }
                    };
                    all_blocks.push(DocxBlock::Image(bytes));
                }
                create_pdf_docx(all_blocks, args.font, args.size, args.paper, args.orientation)
            }
        },
        "txt" => {
            let mut combined = String::new();
            for path in args.input {
                let mut file = match File::open(&path) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Error opening file {:?}: {}", path, e);
                        process::exit(1);
                    }
                };
                let mut data = String::new();
                if let Err(e) = file.read_to_string(&mut data) {
                    eprintln!("Error reading file {:?}: {}", path, e);
                    process::exit(1);
                }
                combined.push_str(&data);
                combined.push('\x0C'); 
            }
            create_pdf_text(combined, args.font, args.size, args.paper, args.orientation)
        },
        "docx" => {
            let mut all_blocks: Vec<DocxBlock> = Vec::new();
            for path in args.input {
                let mut blocks = read_docx(path);
                all_blocks.append(&mut blocks);
                all_blocks.push(DocxBlock::PageBreak);
            }
            create_pdf_docx(all_blocks, args.font, args.size, args.paper, args.orientation)
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