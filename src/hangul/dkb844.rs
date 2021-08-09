use core::usize;

use crate::common::{CharacterRenderer, RenderFailureReason};

use super::HangulSyllable;

// Rendering data.
// choLookup1, 2: Key is jungIdx
// jungLookup   : Key is choIdx
// jongLookup   : Key is jongIdx
// Idx               0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
static CHO_LOOKUP1: [usize; 22] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 1, 2, 4, 4, 4, 2, 1, 3, 0];
static CHO_LOOKUP2: [usize; 22] = [0, 5, 5, 5, 5, 5, 5, 5, 5, 6, 7, 7, 7, 6, 6, 7, 7, 7, 6, 6, 7, 5];
static JUNG_LOOKUP1: [usize; 20] = [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1];
static JUNG_LOOKUP2: [usize; 20] = [0, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 3, 3, 3];
static JONG_LOOKUP: [usize; 22] = [0, 0, 2, 0, 2, 1, 2, 1, 2, 3, 0, 2, 1, 3, 3, 1, 2, 1, 3, 3, 1, 1];
static COMPAT_CHO_LOOKUP: [usize; 30] = [1, 2, 0, 3, 0, 0, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 7, 8, 9, 0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];

pub(crate) enum Dkb844HangulLUT {
    ChoLookup1,
    ChoLookup2,
    JungLookup1,
    JungLookup2,
    JongLookup,
    CompatChoIdxLookup,
}

fn dkb844_lookup(table_id: Dkb844HangulLUT, idx: usize) -> Result<usize, RenderFailureReason> { // Well, possible err is obvious here, isn't it?
    match table_id {
        Dkb844HangulLUT::ChoLookup1 if CHO_LOOKUP1.len() > idx => {
            Ok(CHO_LOOKUP1[idx])
        },
        Dkb844HangulLUT::ChoLookup2 if CHO_LOOKUP2.len() > idx => {
            Ok(CHO_LOOKUP2[idx])
        },
        Dkb844HangulLUT::JungLookup1 if JUNG_LOOKUP1.len() > idx => {
            Ok(JUNG_LOOKUP1[idx])
        },
        Dkb844HangulLUT::JungLookup2 if JUNG_LOOKUP2.len() > idx => {
            Ok(JUNG_LOOKUP2[idx])
        },
        Dkb844HangulLUT::JongLookup if JONG_LOOKUP.len() > idx => {
            Ok(JONG_LOOKUP[idx])
        },
        Dkb844HangulLUT::CompatChoIdxLookup if COMPAT_CHO_LOOKUP.len() > idx => {
            Ok(COMPAT_CHO_LOOKUP[idx])
        },
        _ => Err(RenderFailureReason::UnsupportedCharacter),
    }
}

pub struct Dkb844<'a> {
    rom: &'a[u8],
    width: usize,
    height: usize,
    char_sz: usize,
}

impl<'a> Dkb844<'a> {
    pub fn new(rom: &'a[u8], width: usize, height: usize) -> Dkb844<'a> {
        Dkb844 {
            rom,
            width,
            height,
            char_sz: (width + 7) / 8 * height,
        }
    }

    fn getoff(&self, syllable: HangulSyllable, set: usize, charoff: usize) -> usize {
        match syllable {
            HangulSyllable::Choseong => {
                (self.char_sz * 20) * set + self.char_sz * charoff
            },
            HangulSyllable::Jungseong => {
                let baseoff = self.getoff(HangulSyllable::Choseong, 8, 0);
                baseoff + (self.char_sz * 22) * set + self.char_sz * charoff
            },
            HangulSyllable::Jongseong => {
                let baseoff = self.getoff(HangulSyllable::Jungseong, 4, 0);
                baseoff + (self.char_sz * 28) * set + self.char_sz * charoff
            },
        }
    }

    fn decompose_char(&self, character: char) -> Result<(usize, usize, usize), RenderFailureReason> {
        // Unless future people f*cks the Unicode space with cursed Emoji or
        // untli we succenly find lots of alien civilizaitons with unique
        // writing systems, I think u32 will be fine until next century...
        let codept: u32 = character as u32;
        match codept {
            0x1100..=0x1112 => Ok(((codept - 0x1100 + 1) as usize, 0, 0)),
            0x1161..=0x1175 => Ok((0, (codept - 0x1161 + 1) as usize, 0)),
            0x11A8..=0x11C2 => Ok((0, 0, (codept - 0x11A8 + 1) as usize)),
            0x3131..=0x314E => {
                match dkb844_lookup(Dkb844HangulLUT::CompatChoIdxLookup, (codept - 0x3131) as usize) {
                    Ok(idx) => Ok((idx, 0, 0)),
                    Err(x) => Err(x),
                }
            },
            0x314F..=0x3163 => Ok((0, (codept - 0x314F + 1) as usize, 0)),
            0xAC00..=0xD7A3 => {
                let nchr = codept - 0xAC00;
                    let cho_idx = nchr / (0x0015 * 0x001C) + 1;
                let jung_idx = (nchr / 0x001C) % 0x0015 + 1;
                let jong_idx = nchr % 0x001C;

                Ok((cho_idx as usize, jung_idx as usize, jong_idx as usize))
            }
            _ => Err(RenderFailureReason::UnsupportedCharacter)
        }
    }

    fn choose_set(&self, tup: (usize, usize, usize)) -> Result<(usize, usize, usize), RenderFailureReason> {
        match tup {
            (0, 0, 0) => Ok((0, 0, 0)),
            (_, 0, 0) => Ok((1, 0, 0)),
            (0, _, 0) => Ok((0, 0, 0)),
            (0, 0, _) => Ok((0, 0, 0)),
            (cho_idx, jung_idx, 0) => {
                let cho_set = dkb844_lookup(Dkb844HangulLUT::ChoLookup1, jung_idx)?;
                let jung_set = dkb844_lookup(Dkb844HangulLUT::JungLookup1, cho_idx)?;
                Ok((cho_set, jung_set, 0))
            },
            (cho_idx, jung_idx, _) => {
                let cho_set = dkb844_lookup(Dkb844HangulLUT::ChoLookup2, jung_idx)?;
                let jung_set = dkb844_lookup(Dkb844HangulLUT::JungLookup2, cho_idx)?;
                let jong_set = dkb844_lookup(Dkb844HangulLUT::JongLookup, jung_idx)?;
                Ok((cho_set, jung_set, jong_set))
            }
        }
    }
}

impl<'a> CharacterRenderer for Dkb844<'a> {
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason> {
        let decomposed = self.decompose_char(character)?;
        let set = self.choose_set(decomposed)?;

        let off: [usize; 3] = [
            self.getoff(HangulSyllable::Choseong, set.0, decomposed.0),
            self.getoff(HangulSyllable::Jungseong, set.1, decomposed.1),
            self.getoff(HangulSyllable::Jongseong, set.2, decomposed.2),
        ];

        // check given buf has enough size for character
        if buf.len() < self.char_sz {
            return Err(RenderFailureReason::NotEnoughBuffer);
        }

        // Zero-fill &buf
        buf[..self.char_sz].fill(0x00);

        for idx in 0..3 {
            let syllable = off[idx];
            for (off, elem) in buf.iter_mut().enumerate().take(self.char_sz) {
                *elem |= self.rom[syllable + off];
            }
        }

        Ok((self.width, self.height))
    }
}
