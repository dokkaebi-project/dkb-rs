use crate::common::FontRenderer;

// Rendering data.
// choLookup1, 2: Key is jungIdx
// jungLookup   : Key is choIdx
// jongLookup   : Key is jungIdx
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
        Dkb844HangulLUT::ChoLookup1 => {
            if CHO_LOOKUP1.len() < idx { Err(()) } else { Ok(CHO_LOOKUP1[idx]) }
        },
        Dkb844HangulLUT::ChoLookup2 => {
            if CHO_LOOKUP2.len() < idx { Err(()) } else { Ok(CHO_LOOKUP2[idx]) }
        },
        Dkb844HangulLUT::JungLookup1 => {
            if JUNG_LOOKUP1.len() < idx { Err(()) } else { Ok(JUNG_LOOKUP1[idx]) }
        },
        Dkb844HangulLUT::JungLookup2 => {
            if JUNG_LOOKUP2.len() < idx { Err(()) } else { Ok(JUNG_LOOKUP2[idx]) }
        },
        Dkb844HangulLUT::JongLookup => {
            if JONG_LOOKUP.len() < idx { Err(()) } else { Ok(JONG_LOOKUP[idx]) }
        },
        Dkb844HangulLUT::CompatChoIdxLookup => {
            if COMPAT_CHO_LOOKUP.len() < idx { Err(()) } else { Ok(COMPAT_CHO_LOOKUP[idx]) }
        },
    }
}

pub(crate) struct Dkb844 {
    //
}

impl FontRenderer for Dkb844 {
    //
}

impl Dkb844 {
    pub fn new() -> Dkb844 {
        Dkb844 {
            //
        }
    }
}
