use std::{fs::File, io::Read, path::PathBuf, process};
use docx_rs::{self, Paragraph, ParagraphChild, RunChild, DocumentChild, TableChild, TableRowChild, TableCellContent, Table, DrawingData};

pub enum DocxBlock {
    Text(String),
    Image(Vec<u8>),
    PageBreak,
}

pub fn read_docx(path: PathBuf) -> Vec<DocxBlock> {
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

    let mut blocks: Vec<DocxBlock> = Vec::new();
    let mut current_text = String::new();

    for child in docx.document.children {
        match child {
            DocumentChild::Paragraph(para) => {
                extract_paragraph(&para, &mut current_text, &mut blocks);
                current_text.push('\n');
            },
            DocumentChild::Table(table) => {
                extract_table(*table, &mut current_text, &mut blocks);
            },
            _ => {}
        }
    }

    if !current_text.is_empty() {
        blocks.push(DocxBlock::Text(current_text));
    }

    blocks
}

fn extract_paragraph(para: &Paragraph, current_text: &mut String, blocks: &mut Vec<DocxBlock>) {
    for child in &para.children {
        if let ParagraphChild::Run(run) = child {
            for child in &run.children {
                match child {
                    RunChild::Text(t) => {
                        current_text.push_str(&t.text);
                    },
                    RunChild::Break(_) => {
                        if !current_text.is_empty() {
                            blocks.push(DocxBlock::Text(current_text.clone()));
                            current_text.clear();
                        }
                        blocks.push(DocxBlock::PageBreak);
                    },
                    RunChild::Drawing(d) => {
                        if !current_text.is_empty() {
                            blocks.push(DocxBlock::Text(current_text.clone()));
                            current_text.clear();
                        }
                        if let Some(data) = &d.data {
                            match data {
                                DrawingData::Pic(pic) => {
                                    if !pic.image.is_empty() {
                                        blocks.push(DocxBlock::Image(pic.image.clone()));
                                    }
                                },
                                DrawingData::TextBox(_) => {},
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
    }
}

#[allow(irrefutable_let_patterns)]
fn extract_table(table: Table, current_text: &mut String, blocks: &mut Vec<DocxBlock>) {
    for row in table.rows {
        if let TableChild::TableRow(row) = row {
            let mut cell_texts = Vec::new();

            for cell in row.cells {
                if let TableRowChild::TableCell(cell) = cell {
                    let mut cell_text = String::new();

                    for child in cell.children {
                        match child {
                            TableCellContent::Paragraph(para) => {
                                extract_paragraph(&para, &mut cell_text, blocks);
                            },
                            TableCellContent::Table(nested_table) => {
                                extract_table(nested_table, &mut cell_text, blocks);
                            },
                            _ => {}
                        }
                    }

                    cell_texts.push(cell_text);
                }
            }

            current_text.push_str(&cell_texts.join(" | "));
            current_text.push('\n');
        }
    }
    current_text.push('\n');
}

