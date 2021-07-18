use core::usize;

use crate::common::CharacterRenderer;

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

fn dkb844_lookup(table_id: Dkb844HangulLUT, idx: usize) -> Result<usize, ()> { // Well, possible err is obvious here, isn't it?
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
        _ => Err(()),
    }
}

pub(crate) struct Dkb844 {
    //
}

impl Dkb844 {
    pub fn new() -> Dkb844 {
        Dkb844 {
            //
        }
    }

    fn decompose_char(&self, character: char) -> Option<(usize, usize, usize)> {
        // Unless future people f*cks the Unicode space with cursed Emoji or
        // untli we succenly find lots of alien civilizaitons with unique
        // writing systems, I think u32 will be fine until next century...
        let codept: u32 = character as u32;
        match codept {
            0x1100..=0x1112 => Some(((codept - 0x1100 + 1) as usize, 0, 0)),
            0x1161..=0x1175 => Some((0, (codept - 0x1161 + 1) as usize, 0)),
            0x11A8..=0x11C2 => Some((0, 0, (codept - 0x11A8 + 1) as usize)),
            0x3131..=0x314E => {
                match dkb844_lookup(Dkb844HangulLUT::CompatChoIdxLookup, (codept - 0x3131) as usize) {
                    Ok(idx) => Some((idx, 0, 0)),
                    Err(_) => None,
                }
            },
            0x314F..=0x3163 => Some((0, (codept - 0x314F + 1) as usize, 0)),
            0xAC00..=0xD7A3 => {
                let nchr = codept - 0xAC00;
                let cho_idx = nchr / (0x0015 * 0x001C) + 1;
                let jung_idx = (nchr / 0x001C) % 0x0015 + 1;
                let jong_idx = nchr % 0x001C;

                Some((cho_idx as usize, jung_idx as usize, jong_idx as usize))
            }
            _ => None
        }
    }
}

impl CharacterRenderer for Dkb844 {
    fn render(&self, character: char, buf: &[u8]) -> Option<&u8> {
        let decomposed = match self.decompose_char(character) {
            Some(tup) => tup,
            None => return None,
        };

        None
    }
}
