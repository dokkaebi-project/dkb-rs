mod fnt;
use crate::common::RenderFailureReason;

use super::common::CharacterRenderer;

pub enum LatinFontFormat {
    FNT,
}

pub struct LatinRenderer {
    width: usize,
    height: usize,
}

impl CharacterRenderer for LatinRenderer {
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason> {
        todo!()
    }
}

impl LatinRenderer {
    pub fn new(width: usize, height: usize) -> LatinRenderer {
        LatinRenderer {
            width,
            height,
        }
    }
}
