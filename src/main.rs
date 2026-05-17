
use printpdf::*;
use std::{fs::File, io::Read, process};

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
    
}