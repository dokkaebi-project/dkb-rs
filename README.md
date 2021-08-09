[![crates.io](https://img.shields.io/crates/d/dkb-rs.svg)](https://crates.io/crates/dkb-rs)
[![crates.io](https://img.shields.io/crates/v/dkb-rs.svg)](https://crates.io/crates/dkb-rs)

# dkb-rs, the retro font renderer which supports Latin, Korean and Japanese. Implemented in Rust

dkb(dokkaebi)-rs, the dokkaebi project reference renderer

This crate is compatible with `no-std`

## Current status

Able to render Latin and Hangul. API is not stable yet.

Documentation in progress

## Currently supported fonts

### Latin
* FNT

### Hangul
* Dokkaebi 844

### Kana and Kanji
* FONTX

## Work in progress fonts
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
