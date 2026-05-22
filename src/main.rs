use std::fs::{File, write};
use std::io::Read;
use std::path::PathBuf;
use std::process;
use converter::create_pdf_text;
use clap::Parser;
mod images;
use crate::images::*;

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

    let path_extension = args.input.extension().unwrap();
    if path_extension == "jpg" || path_extension == "img" || path_extension == "png" || path_extension == "bmp" || path_extension == "webp" || path_extension == "gif" || path_extension == "tiff" || path_extension == "jpeg"
    {
        let pdf_bytes = create_pdf_img(args.input.clone());
         match write(args.output.clone(), pdf_bytes) {
            Ok(_) => println!("Successfully created pdf"),
            Err(e) => {
                eprintln!("Error in creating pdf: {}", e);
                process::exit(1);
            }
        }
    } else {
        let mut file = match File::open(args.input.clone()) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error in getting input text file: {}", e);
                process::exit(1);
            },
        };

        let mut data = String::new();
        if let Err(e) = file.read_to_string(&mut data) {
            eprintln!("Error: {}", e);
            process::exit(1)
        };

        let pdf_bytes = create_pdf_text(data, args.font, args.size, args.paper, args.orientation);

        match write(args.output.clone(), pdf_bytes) {
            Ok(_) => println!("Successfully created pdf"),
            Err(e) => {
                eprintln!("Error in creating pdf: {}", e);
                process::exit(1);
            }
        }
    }

    
}