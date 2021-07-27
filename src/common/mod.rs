pub trait CharacterRenderer {
    // Returns (width, height)
    fn render(&self, character: char, buf: &mut [u8]) -> Option<(usize, usize)>;
}
