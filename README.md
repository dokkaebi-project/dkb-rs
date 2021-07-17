# Bitmapfont-rs, the retro font renderer which supports Latin, Korean and Japanese. Implemented in Rust

Font renderer which supports Latin and CJK

This crate is compatible with `no-std`

## Currently supported fonts

* Latin
* Hangul - Dokkaebi 844

## Work in progress fonts
### Kana and Kanji

* FONTX fonts

Will require pre-processing because UTF-8 to Shift-JIS conversion table
will eat up precious `.text` space and will introduce conversion overhead,
which will render this library not suitable for embedded system use

### Cyrillic

Research in progress, Found 8x8 font binary in Poisk(Поиск) ROM image
but still finding suitable font format in 8x16

## Fonts which require lots of research

### Hanzi (Chinese)

I could not find any Hanzi bitmap font in "old format" yet.

## Fonts that will not be supported

### Hanja

Since the modern Korean language does not use Hanja frequently, there is
no point to invest time to research Hanja support.

### Hangul

* Dokkaebi font in 10x4x4 format
* Dokkaebi font in 10x6x4 format

Their existence is known in [This document](https://wiki.kldp.org/wiki.php/%C1%B6%C7%D5%B1%DB%B2%C3)
but I cannot find any further information on the web.
