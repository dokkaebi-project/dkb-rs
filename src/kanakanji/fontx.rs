use crate::common::{CharacterRenderer, RenderFailureReason};

pub struct FONTX<'a> {
    rom: &'a[u8],
}

impl CharacterRenderer for FONTX<'_> {
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason> {
        todo!()
    }
}

impl<'a> FONTX<'a> {
    pub fn new(rom: &'a[u8]) -> FONTX {
        FONTX {
            rom,
        }
    }
}
