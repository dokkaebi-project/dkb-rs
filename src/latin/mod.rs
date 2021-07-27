mod fnt;
use super::common::CharacterRenderer;

pub enum LatinFontFormat {
    FNT,
}

pub struct LatinRenderer {
    width: usize,
    height: usize,
}

impl CharacterRenderer for LatinRenderer {
    fn render(&self, character: char, buf: &mut [u8]) -> Option<(usize, usize)> {
        None
    }
}

impl LatinRenderer {
    pub fn new(width: usize, height: usize) -> LatinRenderer {
        LatinRenderer {
            width,
            height,
        }
    }
}
