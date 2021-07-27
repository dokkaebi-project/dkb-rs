use crate::common::CharacterRenderer;

pub(crate) struct FONTX {
    //
}

impl CharacterRenderer for FONTX {
    fn render(&self, character: char, buf: &mut [u8]) -> Option<(usize, usize)> {
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
