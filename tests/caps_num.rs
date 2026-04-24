//! Caps Lock and Num Lock correctness — Caps flips letters only, NumLock
//! toggles keypad digit vs navigation semantics.

use sdl_keybridge::{resolve, KeyMod, LabelStyle, MultiLocalizer, NamedKey, Scancode};

#[test]
fn caps_applies_only_to_letters() {
    let loc = MultiLocalizer::new();

    let letter = resolve(
        Scancode::A,
        KeyMod::CAPS,
        "windows/en-t-k0-windows",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(letter.character, Some('A'), "Caps should uppercase letters");

    let digit = resolve(
        Scancode::NUM_1,
        KeyMod::CAPS,
        "windows/en-t-k0-windows",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(
        digit.character,
        Some('1'),
        "Caps must NOT convert digits — '1' must stay '1', not '!'"
    );

    let bracket = resolve(
        Scancode::LEFT_BRACKET,
        KeyMod::CAPS,
        "windows/en-t-k0-windows",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(
        bracket.character,
        Some('['),
        "Caps must NOT shift punctuation"
    );
}

#[test]
fn caps_plus_shift_cancels_on_letters() {
    let loc = MultiLocalizer::new();
    let both = resolve(
        Scancode::A,
        KeyMod::CAPS | KeyMod::LSHIFT,
        "windows/en-t-k0-windows",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(
        both.character,
        Some('a'),
        "Shift+Caps on a letter returns to lowercase"
    );
}

#[test]
fn numlock_on_keypad_produces_digit() {
    let loc = MultiLocalizer::new();
    let r = resolve(
        Scancode::KP_1,
        KeyMod::NUM,
        "windows/en-t-k0-windows",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r.character, Some('1'));
    assert_eq!(r.named_key, Some(NamedKey::Keypad1));
}

#[test]
fn numlock_off_keypad_is_navigation() {
    let loc = MultiLocalizer::new();

    let r1 = resolve(
        Scancode::KP_1,
        KeyMod::NONE,
        "windows/en-t-k0-windows",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r1.named_key, Some(NamedKey::End));
    assert_eq!(r1.character, None);

    let r2 = resolve(
        Scancode::KP_2,
        KeyMod::NONE,
        "windows/en-t-k0-windows",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r2.named_key, Some(NamedKey::ArrowDown));

    let r_period = resolve(
        Scancode::KP_PERIOD,
        KeyMod::NONE,
        "windows/en-t-k0-windows",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r_period.named_key, Some(NamedKey::Delete));
}
