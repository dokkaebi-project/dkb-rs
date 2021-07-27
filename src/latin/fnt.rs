use crate::common::{CharacterRenderer, RenderFailureReason};

pub struct FNT {
    width: usize,
    height: usize,
}

impl CharacterRenderer for FNT {
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason> {
        todo!()
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
