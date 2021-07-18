use crate::common::CharacterRenderer;

pub(crate) struct FONTX {
    //
}

impl CharacterRenderer for FONTX {
    fn render(&self, character: char, buf: &[u8]) -> Option<&u8> {
        None
    }
}

impl FONTX {
    pub fn new() -> FONTX {
        FONTX {
            //
        }
    }
}
