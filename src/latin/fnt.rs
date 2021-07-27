use crate::common::{CharacterRenderer, RenderFailureReason};

pub struct FNT<'a> {
    rom: &'a[u8],
    width: usize,
    height: usize,
}

impl CharacterRenderer for FNT<'_> {
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason> {
        todo!()
    }
}

impl<'a> FNT<'a> {
    pub fn new(rom: &'a[u8], width : usize, height: usize) -> FNT {
        FNT {
            rom,
            width,
            height,
        }
    }
}
