use std::fs::{File, write};
use std::io::Read;
use std::process;
use text_to_pdf::create_pdf;

fn main() {
    let mut file = match File::open("index.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Error due to : {}", e);
            process::exit(1);
        },
    };

    let mut data = String::new();
    if let Err(e) = file.read_to_string(&mut data) {
        println!("Error caused due to : {}", e);
        process::exit(1)
    };

    let pdf_bytes = create_pdf(data);

    match write("output.pdf", pdf_bytes) {
        Ok(_) => println!("Successfully created pdf"),
        Err(e) => {
            println!("Error creating pdf: {}", e);
            process::exit(1);
        }
    }
}