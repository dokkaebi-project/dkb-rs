pub enum RenderFailureReason {
    UnsupportedCharacter,
    NotEnoughBuffer,
}

pub trait CharacterRenderer {
    // Returns (width, height)
    fn render(&self, character: char, buf: &mut [u8]) -> Result<(usize, usize), RenderFailureReason>;
}

pub trait StringRenderer {
}
