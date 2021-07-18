mod fnt;
use super::common::CharacterRenderer;

pub enum LatinFontFormat {
    FNT,
}

pub struct LatinRenderer {
    //
}

impl CharacterRenderer for LatinRenderer {
    fn render(&self, character: char, buf: &[u8]) -> Option<&u8> {
        None
    }
}

impl LatinRenderer {
    pub fn new() -> LatinRenderer {
        LatinRenderer {
            //
        }
    }
}
