use crate::common::{CharacterRenderer, RenderFailureReason};

pub struct FONTX {
    //
}

impl CharacterRenderer for FONTX {
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason> {
        todo!()
    }
}

impl FONTX {
    pub fn new() -> FONTX {
        FONTX {
            //
        }
    }
}
