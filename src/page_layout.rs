use printpdf::*;
use std::str::FromStr;

pub struct PDim {
    pub width: Mm,
    pub height: Mm,
    pub top_margin: f32,
    pub bottom_margin: f32,
    pub left_margin: f32,
    pub right_margin: f32,
}

impl PDim {
    pub fn landscape(self) -> PDim {
        let new_height_pt = self.width.0 as f32 * 2.8346;
        let new_width_pt = self.height.0 as f32 * 2.8346;
        let margin = 50.0;
        PDim {
            width: self.height,
            height: self.width,
            top_margin: new_height_pt - margin,
            bottom_margin: margin,
            left_margin: margin,
            right_margin: new_width_pt - margin,
        }
    }
}

pub enum PSize {
    A4,
    A3,
    Letter,
    Legal,
}

impl PSize {
    pub fn dimensions(&self) -> PDim {
        match self {
            PSize::A4 => {
                let height_pt = 842.0;
                let margin = 50.0;
                PDim {
                    width: Mm(210.0),
                    height: Mm(297.0),
                    top_margin: height_pt - margin,
                    bottom_margin: margin,
                    left_margin: margin,
                    right_margin: 210.0 * 2.8346 - margin,
                }
            },
            PSize::A3 => {
                let height_pt = 1191.0;
                let margin = 50.0;
                PDim {
                    width: Mm(297.0),
                    height: Mm(420.0),
                    top_margin: height_pt - margin,
                    bottom_margin: margin,
                    left_margin: margin,
                    right_margin: 297.0 * 2.8346 - margin,
                }
            },
            PSize::Letter => {
                let height_pt = 792.0;
                let margin = 50.0;
                PDim {
                    width: Mm(215.9),
                    height: Mm(279.4),
                    top_margin: height_pt - margin,
                    bottom_margin: margin,
                    left_margin: margin,
                    right_margin: 215.9 * 2.8346 - margin,
                }
            },
            PSize::Legal => {
                let height_pt = 1008.0;
                let margin = 50.0;
                PDim {
                    width: Mm(215.9),
                    height: Mm(355.6),
                    top_margin: height_pt - margin,
                    bottom_margin: margin,
                    left_margin: margin,
                    right_margin: 215.9 * 2.8346 - margin,
                }
            },
        }
    }
}

impl FromStr for PSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a4"     => Ok(PSize::A4),
            "a3"     => Ok(PSize::A3),
            "letter" => Ok(PSize::Letter),
            "legal"  => Ok(PSize::Legal),
            _ => Err(format!("Unknown paper size: {}. Valid options: a4, a3, letter, legal", s)),
        }
    }
}