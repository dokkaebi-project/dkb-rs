mod dkb844;
use super::common::CharacterRenderer;

pub enum HangulFontFormat {
    DKB844,
}

pub struct HangulRenderer {

}

impl CharacterRenderer for HangulRenderer {
    fn render(&self, character: char, buf: &[u8]) -> Option<&u8> {
        None
    }
}

impl HangulRenderer {
    pub fn new() -> HangulRenderer {
        HangulRenderer {
            //
        }
    }
}
