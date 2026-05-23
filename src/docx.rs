use std::{fs::File, io::Read, path::PathBuf, process};
use docx_rs::*;

pub fn read_docx(path: PathBuf) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error in reading file: {}", e);
            process::exit(1);
        }
    };

    let mut buf = Vec::new();
    match file.read_to_end(&mut buf) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading file contents: {}", e);
            process::exit(1);
        }
    };

    let docx = match docx_rs::read_docx(&buf) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error parsing docx: {}", e);
            process::exit(1);
        }
    };

    let mut text = String::new();

    for child in docx.document.children {
        match child {
            DocumentChild::Paragraph(para) => {
                extract_paragraph(&para, &mut text);
                text.push('\n');
            },
            DocumentChild::Table(table) => {
                extract_table(*table, &mut text);
            },
            _ => {}
        }
    }

    text
}

fn extract_paragraph(para: &Paragraph, text: &mut String) {
    for child in &para.children {
        if let ParagraphChild::Run(run) = child {
            for child in &run.children {
                if let RunChild::Text(t) = child {
                    text.push_str(&t.text);
                }
            }
        }
    }
}

#[allow(irrefutable_let_patterns)]
fn extract_table(table: Table, text: &mut String) {
    for row in table.rows {
        if let TableChild::TableRow(row) = row {
            let mut cell_texts = Vec::new();
            
            for cell in row.cells {
                if let TableRowChild::TableCell(cell) = cell {
                    let mut cell_text = String::new();

                    for child in cell.children {
                        match child {
                            TableCellContent::Paragraph(para) => {
                                extract_paragraph(&para, &mut cell_text);
                            },
                            TableCellContent::Table(nested_table) => {
                                extract_table(nested_table, &mut cell_text);
                            },
                            _ => {}
                        }
                    }

                    cell_texts.push(cell_text);
                }
            }

            text.push_str(&cell_texts.join(" | "));
            text.push('\n');
        }
    }
    text.push('\n');
}