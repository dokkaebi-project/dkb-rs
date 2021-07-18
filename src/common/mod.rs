pub trait CharacterRenderer {
    fn render(&self, character: char, buf: &[u8]) -> Option<&u8>;
}
