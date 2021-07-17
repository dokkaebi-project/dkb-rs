#![no_std]
mod common;
mod hangul;
mod kanakanji;
mod latin;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
