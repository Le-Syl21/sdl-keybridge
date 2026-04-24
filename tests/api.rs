//! Smoke tests for the four public API functions.

use sdl_keybridge::{
    keycode_from_name, modifier_label, resolve, scancode_for, KeyMod, Keycode, LabelStyle,
    Modifier, MultiLocalizer, NamedKey, Platform, Scancode,
};

#[test]
fn resolve_letter_us_qwerty() {
    let loc = MultiLocalizer::new();
    let r = resolve(
        Scancode::A,
        KeyMod::NONE,
        "linux/en-US-t-k0-qwerty",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r.character, Some('a'));
    assert_eq!(r.keycode, Keycode::A);
    assert_eq!(r.glyph_local.as_ref(), "a");
    assert!(r.named_key.is_none());
}

#[test]
fn resolve_letter_with_shift() {
    let loc = MultiLocalizer::new();
    let r = resolve(
        Scancode::A,
        KeyMod::LSHIFT,
        "linux/en-US-t-k0-qwerty",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r.character, Some('A'));
    assert_eq!(r.keycode, Keycode::A); // base keycode stays lowercase
}

#[test]
fn resolve_azerty_q_gives_a() {
    let loc = MultiLocalizer::new();
    let r = resolve(
        Scancode::Q,
        KeyMod::NONE,
        "linux/fr-t-k0-azerty",
        "fr",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r.character, Some('a'));
    assert_eq!(r.keycode, Keycode::A);
}

#[test]
fn resolve_azerty_digit_row_shift_gives_digit() {
    // On AZERTY, unshifted 1-key is '&'; Shift gives '1'.
    let loc = MultiLocalizer::new();
    let r_base = resolve(
        Scancode::NUM_1,
        KeyMod::NONE,
        "linux/fr-t-k0-azerty",
        "fr",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r_base.character, Some('&'));

    let r_shift = resolve(
        Scancode::NUM_1,
        KeyMod::LSHIFT,
        "linux/fr-t-k0-azerty",
        "fr",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r_shift.character, Some('1'));
}

#[cfg(feature = "en")]
#[test]
fn resolve_named_key_is_localized_en() {
    let loc = MultiLocalizer::new();
    let r_en = resolve(
        Scancode::ESCAPE,
        KeyMod::NONE,
        "linux/en-US-t-k0-qwerty",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r_en.named_key, Some(NamedKey::Escape));
    assert_eq!(r_en.glyph_en.as_ref(), "Esc");
}

#[cfg(feature = "fr")]
#[test]
fn resolve_named_key_is_localized_fr() {
    let loc = MultiLocalizer::new();
    let r_fr = resolve(
        Scancode::ESCAPE,
        KeyMod::NONE,
        "linux/fr-t-k0-azerty",
        "fr",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r_fr.glyph_local.as_ref(), "Échap");
}

#[cfg(feature = "en")]
#[test]
fn resolve_arrow_symbolic_vs_textual() {
    let loc = MultiLocalizer::new();
    let sym = resolve(
        Scancode::UP,
        KeyMod::NONE,
        "linux/en-US-t-k0-qwerty",
        "en",
        LabelStyle::Symbolic,
        &loc,
    );
    assert_eq!(sym.glyph_local.as_ref(), "↑");

    let txt = resolve(
        Scancode::UP,
        KeyMod::NONE,
        "linux/en-US-t-k0-qwerty",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(txt.glyph_local.as_ref(), "Up");
}

#[test]
fn scancode_for_round_trip_us_qwerty() {
    for c in ['a', 'z', 'q', 'm', '1', '0', '-', '='] {
        let sc = scancode_for(Keycode::from(c), "linux/en-US-t-k0-qwerty")
            .unwrap_or_else(|| panic!("no scancode for {c:?}"));
        let loc = MultiLocalizer::new();
        let r = resolve(
            sc,
            KeyMod::NONE,
            "linux/en-US-t-k0-qwerty",
            "en",
            LabelStyle::Textual,
            &loc,
        );
        assert_eq!(r.character, Some(c), "round-trip failed for {c:?}");
    }
}

#[test]
fn scancode_for_named_key() {
    let sc = scancode_for(Keycode::ESCAPE, "linux/en-US-t-k0-qwerty");
    assert_eq!(sc, Some(Scancode::ESCAPE));

    let sc = scancode_for(Keycode::F5, "linux/en-US-t-k0-qwerty");
    assert_eq!(sc, Some(Scancode::F5));
}

#[cfg(feature = "en")]
#[test]
fn modifier_label_platform_aware() {
    let loc = MultiLocalizer::new();

    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Mac,
            "en",
            LabelStyle::Symbolic,
            &loc
        )
        .as_ref(),
        "⌘"
    );
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Windows,
            "en",
            LabelStyle::Symbolic,
            &loc
        )
        .as_ref(),
        "⊞"
    );
    assert_eq!(
        modifier_label(
            Modifier::Alt,
            Platform::Mac,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Option"
    );
    assert_eq!(
        modifier_label(
            Modifier::Alt,
            Platform::Windows,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Alt"
    );
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Linux,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Super"
    );
}

#[test]
#[cfg(feature = "fr")]
fn modifier_label_localized_fr() {
    let loc = MultiLocalizer::new();
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Mac,
            "fr",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Commande"
    );
    assert_eq!(
        modifier_label(
            Modifier::Alt,
            Platform::Mac,
            "fr",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Option"
    );
    assert_eq!(
        modifier_label(
            Modifier::Ctrl,
            Platform::Windows,
            "fr",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Ctrl"
    );
}

#[test]
fn keycode_from_name_round_trips() {
    assert_eq!(keycode_from_name("Escape"), Some(Keycode::ESCAPE));
    assert_eq!(keycode_from_name("Esc"), Some(Keycode::ESCAPE));
    assert_eq!(keycode_from_name("F5"), Some(Keycode::F5));
    assert_eq!(keycode_from_name("Left Shift"), Some(Keycode::LSHIFT));
    assert_eq!(keycode_from_name("a"), Some(Keycode::A));
    assert_eq!(keycode_from_name("1"), Some(Keycode(b'1' as u32)));
    assert_eq!(keycode_from_name("Tab"), Some(Keycode::TAB));
    assert_eq!(keycode_from_name("nonsense42"), None);
}

#[test]
fn keycode_from_name_case_insensitive_for_letters() {
    assert_eq!(keycode_from_name("A"), Some(Keycode::A));
    assert_eq!(keycode_from_name("ESCAPE"), Some(Keycode::ESCAPE));
    assert_eq!(keycode_from_name("f5"), Some(Keycode::F5));
}

#[test]
fn unknown_layout_falls_back_to_us_qwerty() {
    let loc = MultiLocalizer::new();
    let r = resolve(
        Scancode::A,
        KeyMod::NONE,
        "linux/xyz-nonsense",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r.character, Some('a'));
}
