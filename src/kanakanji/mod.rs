mod fontx;
use crate::common::RenderFailureReason;

use super::common::CharacterRenderer;

pub enum KanaKanjiFontFormat {
    FONTX,
}

pub struct KanaKanjiRenderer {
    //
}

impl CharacterRenderer for KanaKanjiRenderer {
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason> {
        todo!();
    }
}

impl KanaKanjiRenderer {
    pub fn new() -> KanaKanjiRenderer {
        KanaKanjiRenderer {
            //
        }
    }
}
