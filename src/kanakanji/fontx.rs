use encoding_rs::{EncoderResult, SHIFT_JIS};

use crate::common::{CharacterRenderer, RenderFailureReason};

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
struct FontXHeader {
    magic: [u8; 6], // Should be "FONTX2"
    name: [u8; 8],  // 8-byte &str
    width: u8,      // Font width (dots)
    height: u8,     // Font height (dots)
    code_flag: u8,  // 0: ANK, 1: Shift-JIS
    codeblocks: u8, // Number of code blocks, this does not exist in ANK font
}

enum FONTXCode {
    ANK,
    ShiftJIS,
}

pub struct FONTX<'a> {
    rom: &'a[u8],
    code: FONTXCode,
    width: usize,
    height: usize,
    char_sz: usize,
    codeblocks: usize,
    headersz: usize,
}

impl CharacterRenderer for FONTX<'_> {
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason> {
        if character as u32 > 0xFFFF {
            return Err(RenderFailureReason::UnsupportedCharacter);
        }

        let off = self.get_sjis_offset(character)?;
        for idx in 0..self.char_sz {
            buf[idx] = self.rom[off + idx];
        }

        Ok((self.width, self.height))
    }
}

impl<'a> FONTX<'a> {
    pub fn new(rom: &'a[u8]) -> Result<FONTX, ()> {
        let mut header: FontXHeader = FontXHeader {
            magic: [0; 6],
            name: [0; 8],
            width: 0,
            height: 0,
            code_flag: 0,
            codeblocks: 0,
        };

        unsafe {
            let tmp = &mut header as *mut _ as *mut u8;
            tmp.copy_from(rom as *const _ as *const u8, 18);
        }

        if header.magic != [0x46, 0x4f, 0x4e, 0x54, 0x58, 0x32] {
            return Err(());
        }

        let code = match header.code_flag {
            0 => FONTXCode::ANK,
            1 => FONTXCode::ShiftJIS,
            _ => return Err(()),
        };

        Ok(FONTX {
            codeblocks: match code { FONTXCode::ANK => 0, _ => header.codeblocks as usize },
            headersz: match code { FONTXCode::ANK => 17, _ => 18 + (header.codeblocks as usize) * 4 },
            rom,
            code,
            width: header.width as usize,
            height: header.height as usize,
            char_sz: (header.width as usize + 7) / 8 * header.height as usize,
        })
    }

    pub fn get_sjis_offset(&self, character: char) -> Result<usize, RenderFailureReason> {
        if character as u32 > 0xFFFF {
            return Err(RenderFailureReason::UnsupportedCharacter);
        }

        let code_arr = [character as u16];
        let mut sjis_arr = [0 as u8; 2];
        let mut enc = SHIFT_JIS.new_encoder();
        match enc.encode_from_utf16_without_replacement(&code_arr, &mut sjis_arr, true) {
            (EncoderResult::InputEmpty, _srcsz, _dstsz ) => {
                // Do nothing
            },
            (EncoderResult::OutputFull, _, _) => {
                return Err(RenderFailureReason::UnknownError)
            },
            (EncoderResult::Unmappable(_), _, _) => {
                return Err(RenderFailureReason::UnsupportedCharacter);
            }
        };

        let sjis_code = ((sjis_arr[0] as u16) << 8) + sjis_arr[1] as u16;

        match self.code {
            FONTXCode::ANK => {
                if sjis_code < 0xFF {
                    Ok(self.headersz + (sjis_code as usize) * self.char_sz)
                } else {
                    Err(RenderFailureReason::UnsupportedCharacter)
                }
            },
            FONTXCode::ShiftJIS => {
                // Seek the table
                // Code converted from http://elm-chan.org/docs/dosv/fontx_e.html
                let mut charcnt: usize = 0;
                for blk in 0..self.codeblocks {
                    let off = 18 + 4 * blk;
                    let sb: u16 = ((self.rom[off + 1] as u16) << 8) + (self.rom[off] as u16);
                    let eb: u16 = ((self.rom[off + 3] as u16) << 8) + (self.rom[off + 2] as u16);

                    if sb <= sjis_code && eb >= sjis_code {
                        charcnt += (sjis_code - sb) as usize;
                        return Ok(self.headersz + charcnt * self.char_sz)
                    }

                    charcnt += (eb - sb + 1) as usize;
                }

                Err(RenderFailureReason::UnsupportedCharacter)
            }
        }
    }
}
