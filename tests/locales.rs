//! Locale-completeness test — every compiled locale must resolve every
//! `NamedKey::key_id()` to some non-empty string in both Textual and
//! Symbolic styles, through the runtime fallback chain (locale → en).
//!
//! Every test in this file relies on the English fallback for ids that
//! individual locales may legitimately not translate (F-keys, keypad).

#![cfg(feature = "en")]

use sdl_keybridge::{LabelStyle, MultiLocalizer, NamedKey};

fn every_named_key_translates_for_locale(locale: &str) {
    let loc = MultiLocalizer::new();
    for nk in NamedKey::all() {
        for style in [LabelStyle::Textual, LabelStyle::Symbolic] {
            let id = nk.key_id();
            let via_locale = sdl_keybridge::KeyLocalizer::translate(&loc, id, locale, style);
            let via_en = sdl_keybridge::KeyLocalizer::translate(&loc, id, "en", style);
            assert!(
                via_locale.is_some() || via_en.is_some(),
                "no translation for ({id:?}, {locale}, {style:?}) even with en fallback"
            );
            let best = via_locale.or(via_en).unwrap();
            assert!(
                !best.is_empty(),
                "empty translation for ({id:?}, {locale}, {style:?})"
            );
        }
    }
}

#[test]
fn en_covers_every_named_key() {
    every_named_key_translates_for_locale("en");
}

#[cfg(feature = "fr")]
#[test]
fn fr_covers_every_named_key_via_fallback() {
    every_named_key_translates_for_locale("fr");
}

#[cfg(feature = "de")]
#[test]
fn de_covers_every_named_key_via_fallback() {
    every_named_key_translates_for_locale("de");
}

#[cfg(feature = "es")]
#[test]
fn es_covers_every_named_key_via_fallback() {
    every_named_key_translates_for_locale("es");
}

#[cfg(feature = "ja")]
#[test]
fn ja_covers_every_named_key_via_fallback() {
    every_named_key_translates_for_locale("ja");
}

#[cfg(feature = "ru")]
#[test]
fn ru_covers_every_named_key_via_fallback() {
    every_named_key_translates_for_locale("ru");
}

#[cfg(feature = "zh-hans")]
#[test]
fn zh_hans_covers_every_named_key_via_fallback() {
    every_named_key_translates_for_locale("zh-hans");
}

#[cfg(feature = "zh-hans")]
#[test]
fn zh_hans_accepts_bcp47_aliases() {
    let loc = MultiLocalizer::new();
    let key = "key_arrow_up";
    let via_cn = sdl_keybridge::KeyLocalizer::translate(&loc, key, "zh-CN", LabelStyle::Textual);
    let via_hans =
        sdl_keybridge::KeyLocalizer::translate(&loc, key, "zh-hans", LabelStyle::Textual);
    assert_eq!(via_cn, via_hans);
}

#[test]
fn region_subtag_is_stripped() {
    let loc = MultiLocalizer::new();
    let via_en_us =
        sdl_keybridge::KeyLocalizer::translate(&loc, "key_escape", "en-US", LabelStyle::Textual);
    let via_en =
        sdl_keybridge::KeyLocalizer::translate(&loc, "key_escape", "en", LabelStyle::Textual);
    assert_eq!(via_en_us, via_en);
}
