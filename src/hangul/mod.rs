mod dkb844;
use super::common::FontRenderer;

pub enum HangulFontFormat {
    DKB844,
}

pub struct HangulRenderer {

}

impl FontRenderer for HangulRenderer {
    //
}

impl HangulRenderer {
    pub fn new() -> HangulRenderer {
        HangulRenderer {
            //
        }
    }
}