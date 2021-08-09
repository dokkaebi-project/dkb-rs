use core::fmt::Debug;

pub enum RenderFailureReason {
    UnknownError,
    UnsupportedCharacter,
    NotEnoughBuffer,
}

impl Debug for RenderFailureReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RenderFailureReason::UnknownError => {
                f.debug_struct("Render failed: Unknown error occured").finish()
            },
            RenderFailureReason::UnsupportedCharacter => {
                f.debug_struct("Render failed: Unsupported character").finish()
            },
            RenderFailureReason::NotEnoughBuffer => {
                f.debug_struct("Render failed: Not enough buffer").finish()
            },
        }
    }
}

pub trait CharacterRenderer {
    // Returns (width, height)
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason>;
}

pub trait StringRenderer {
}
