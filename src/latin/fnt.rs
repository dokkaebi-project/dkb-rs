use crate::common::CharacterRenderer;

pub(crate) struct FNT {
    width: usize,
    height: usize,
}

impl CharacterRenderer for FNT {
    fn render(&self, character: char, buf: &mut [u8]) -> Option<(usize, usize)> {
        None
    }
}

impl FNT {
    pub fn new(width : usize, height: usize) -> FNT {
        FNT {
            width,
            height,
        }
    }
}
