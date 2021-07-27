mod fontx;
use super::common::CharacterRenderer;

pub enum KanaKanjiFontFormat {
    FONTX,
}

pub struct KanaKanjiRenderer {
    //
}

impl CharacterRenderer for KanaKanjiRenderer {
    fn render(&self, character: char, buf: &mut [u8]) -> Option<(usize, usize)> {
        None
    }
}

impl KanaKanjiRenderer {
    pub fn new() -> KanaKanjiRenderer {
        KanaKanjiRenderer {
            //
        }
    }
}
