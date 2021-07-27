mod dkb844;
use crate::common::RenderFailureReason;

use super::common::CharacterRenderer;

pub use dkb844::Dkb844;

pub enum HangulFontFormat {
    DKB844,
}

pub(crate) enum HangulSyllable {
    Choseong,
    Jungseong,
    Jongseong,
}

pub struct HangulRenderer {

}

impl CharacterRenderer for HangulRenderer {
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason> {
        todo!()
    }
}

impl HangulRenderer {
    pub fn new() -> HangulRenderer {
        HangulRenderer {
            //
        }
    }
}
