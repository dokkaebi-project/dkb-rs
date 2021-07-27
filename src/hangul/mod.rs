mod dkb844;
pub use dkb844::Dkb844;

pub enum HangulFontFormat {
    DKB844,
}

pub(crate) enum HangulSyllable {
    Choseong,
    Jungseong,
    Jongseong,
}
