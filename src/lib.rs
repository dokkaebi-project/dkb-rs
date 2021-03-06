#![no_std]

#[cfg(test)]
extern crate std;

mod common;
pub mod hangul;
pub mod kanakanji;
pub mod latin;

pub use common::CharacterRenderer;
pub use common::RenderFailureReason;
pub struct FontRenderer {
    //
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
