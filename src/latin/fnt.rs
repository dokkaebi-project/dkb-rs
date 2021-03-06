use crate::common::{CharacterRenderer, RenderFailureReason};

pub struct FNT<'a> {
    rom: &'a [u8],
    width: usize,
    height: usize,
    char_sz: usize,
}

impl CharacterRenderer for FNT<'_> {
    fn render(
        &self,
        character: char,
        buf: &mut [u8],
    ) -> Result<(usize, usize), RenderFailureReason> {
        let codept = character as u32;

        if (codept as usize) >= (self.rom.len() / self.char_sz) {
            return Err(RenderFailureReason::UnsupportedCharacter);
        }

        if buf.len() < self.char_sz {
            return Err(RenderFailureReason::NotEnoughBuffer);
        }

        let off = (codept as usize) * self.char_sz;
        buf[..self.char_sz].clone_from_slice(&self.rom[off..(self.char_sz + off)]);

        Ok((self.width, self.height))
    }
}

impl<'a> FNT<'a> {
    pub fn new(rom: &'a [u8], width: usize, height: usize) -> FNT {
        FNT {
            rom,
            width,
            height,
            char_sz: (width + 7) / 8 * height,
        }
    }
}
