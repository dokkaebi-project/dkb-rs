use crate::common::CharacterRenderer;

pub(crate) struct FNT {
    //
}

impl CharacterRenderer for FNT {
    fn render(&self, character: char, buf: &[u8]) -> Option<&u8> {
        None
    }
}

impl FNT {
    pub fn new() -> FNT {
        FNT {
            //
        }
    }
}
