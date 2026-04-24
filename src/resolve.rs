//! The four public API functions — the Rosetta Stone's forward, reverse,
//! modifier, and name-parsing lookups.

use std::borrow::Cow;

use crate::keycode::Keycode;
use crate::keymod::KeyMod;
use crate::layout::{self, Layout, LayoutKey};
use crate::localizer::{self, KeyLocalizer, LabelStyle, Modifier, Platform};
use crate::named_key::NamedKey;
use crate::scancode::Scancode;

/// Every parallel representation of a key press, computed in a single pass.
///
/// Returned by [`resolve`].
#[derive(Clone, Debug)]
pub struct Resolved {
    pub scancode: Scancode,
    pub keycode: Keycode,
    /// Localized glyph in English (`"Esc"`, `"Up"`, `"a"`, …) — style-aware.
    pub glyph_en: Cow<'static, str>,
    /// Localized glyph in the requested locale — style-aware.
    pub glyph_local: Cow<'static, str>,
    /// Produced Unicode character, if the key produces one at the current
    /// modifier level. `None` for named keys like Escape / arrows / F-keys.
    pub character: Option<char>,
    /// Named-key identity, if this key is non-printable. `None` for
    /// character-producing keys.
    pub named_key: Option<NamedKey>,
    /// Layout id echoed back — lets callers correlate results.
    pub layout: &'static str,
}

/// Forward lookup — return every parallel representation of the key press.
///
/// - `scancode`: the physical key.
/// - `mods`: current modifier bitmask (including Caps Lock and Num Lock
///   latches, which are handled correctly — Caps only flips letters,
///   Num Lock toggles keypad digit vs navigation).
/// - `layout`: the layout id — e.g. `"linux/fr-t-k0-azerty"`. An unknown
///   id falls back to `"linux/en-US-t-k0-qwerty"`.
/// - `locale`: BCP 47 tag of the UI language — e.g. `"fr"`, `"en-US"`,
///   `"zh-Hans"`.
/// - `style`: textual (`"Up"`, `"Haut"`) vs symbolic (`↑`).
/// - `localizer`: override for per-key translations; usually
///   [`MultiLocalizer`](crate::MultiLocalizer). Must not be consumed.
pub fn resolve<L: KeyLocalizer>(
    scancode: Scancode,
    mods: KeyMod,
    layout: &str,
    locale: &str,
    style: LabelStyle,
    localizer: &L,
) -> Resolved {
    let layout_ref = layout::get_layout(layout)
        .or_else(|| layout::get_layout("linux/en-US-t-k0-qwerty"))
        .expect("en-US-t-k0-qwerty always present");

    resolve_in_layout(scancode, mods, layout_ref, locale, style, localizer)
}

fn resolve_in_layout<L: KeyLocalizer>(
    scancode: Scancode,
    mods: KeyMod,
    layout: &'static Layout,
    locale: &str,
    style: LabelStyle,
    localizer: &L,
) -> Resolved {
    let key = match layout.key(scancode) {
        Some(k) => k,
        None => {
            return Resolved {
                scancode,
                keycode: Keycode::UNKNOWN,
                glyph_en: Cow::Borrowed(""),
                glyph_local: Cow::Borrowed(""),
                character: None,
                named_key: None,
                layout: layout.id,
            };
        }
    };

    // NumLock handling — applies only to keypad keys.
    if let Some(nk) = key.named {
        if let Some((numlock_on_named, numlock_off_named, numlock_char)) = keypad_behavior(nk) {
            if mods.num() {
                // NumLock ON — keypad produces digit character.
                let character = numlock_char;
                return Resolved {
                    scancode,
                    keycode: layout::named_key_keycode(numlock_on_named),
                    glyph_en: character
                        .map(|c| Cow::Owned(c.to_string()))
                        .unwrap_or_else(|| {
                            named_key_label(numlock_on_named, "en", style, localizer)
                        }),
                    glyph_local: character.map(|c| Cow::Owned(c.to_string())).unwrap_or_else(
                        || named_key_label(numlock_on_named, locale, style, localizer),
                    ),
                    character,
                    named_key: Some(numlock_on_named),
                    layout: layout.id,
                };
            } else {
                // NumLock OFF — keypad produces secondary navigation key.
                return Resolved {
                    scancode,
                    keycode: layout::named_key_keycode(numlock_off_named),
                    glyph_en: named_key_label(numlock_off_named, "en", style, localizer),
                    glyph_local: named_key_label(numlock_off_named, locale, style, localizer),
                    character: None,
                    named_key: Some(numlock_off_named),
                    layout: layout.id,
                };
            }
        }

        // Regular named key (not keypad).
        return Resolved {
            scancode,
            keycode: layout::named_key_keycode(nk),
            glyph_en: named_key_label(nk, "en", style, localizer),
            glyph_local: named_key_label(nk, locale, style, localizer),
            character: None,
            named_key: Some(nk),
            layout: layout.id,
        };
    }

    // Printable key. Pick the glyph at the current modifier level.
    let effective_shift = effective_shift_state(key, mods);
    let effective_altgr = mods.altgr();

    let glyph = pick_glyph(key, effective_shift, effective_altgr);

    // SDL keycode for a printable key is the *base-level* Unicode code
    // point (lowercase for letters) regardless of current modifiers.
    let base_keycode = layout::layout_key_base_keycode(key);

    let character = glyph;
    let glyph_str = glyph.map(|c| c.to_string()).unwrap_or_default();

    Resolved {
        scancode,
        keycode: base_keycode,
        glyph_en: Cow::Owned(glyph_str.clone()),
        glyph_local: Cow::Owned(glyph_str),
        character,
        named_key: None,
        layout: layout.id,
    }
}

/// Pick the glyph from a [`LayoutKey`]'s 4-level modifier table.
fn pick_glyph(key: &LayoutKey, shift: bool, altgr: bool) -> Option<char> {
    match (shift, altgr) {
        (false, false) => key.base,
        (true, false) => key.shift.or(key.base),
        (false, true) => key.altgr.or(key.base),
        (true, true) => key.shift_altgr.or(key.shift).or(key.base),
    }
    .and_then(|c| if c == '\0' { None } else { Some(c) })
}

/// Determine whether Shift is effectively applied to *this* key, taking
/// Caps Lock into account — Caps only flips letters.
fn effective_shift_state(key: &LayoutKey, mods: KeyMod) -> bool {
    let shift = mods.shift();
    let caps_applies = mods.caps() && is_letter_key(key);
    shift ^ caps_applies
}

/// True if the key's base glyph is an alphabetic character (a-z in any
/// script for which `char::is_alphabetic` returns true).
fn is_letter_key(key: &LayoutKey) -> bool {
    key.base.map(|c| c.is_alphabetic()).unwrap_or(false)
}

/// For keypad named keys, return the NumLock-aware triple
/// `(numlock_on_named, numlock_off_named, numlock_char)`.
///
/// The NumLock ON form keeps the keypad identity and carries the digit /
/// period character; the NumLock OFF form switches to the secondary
/// navigation role.
fn keypad_behavior(nk: NamedKey) -> Option<(NamedKey, NamedKey, Option<char>)> {
    use NamedKey::*;
    Some(match nk {
        Keypad0 => (Keypad0, Insert, Some('0')),
        Keypad1 => (Keypad1, End, Some('1')),
        Keypad2 => (Keypad2, ArrowDown, Some('2')),
        Keypad3 => (Keypad3, PageDown, Some('3')),
        Keypad4 => (Keypad4, ArrowLeft, Some('4')),
        Keypad5 => (Keypad5, Keypad5, Some('5')), // 5 has no nav counterpart
        Keypad6 => (Keypad6, ArrowRight, Some('6')),
        Keypad7 => (Keypad7, Home, Some('7')),
        Keypad8 => (Keypad8, ArrowUp, Some('8')),
        Keypad9 => (Keypad9, PageUp, Some('9')),
        KeypadPeriod => (KeypadPeriod, Delete, Some('.')),
        _ => return None,
    })
}

/// Look up the locale + style label for a named key, with fallback
/// chain: user localizer → requested locale → English → raw id.
fn named_key_label<L: KeyLocalizer>(
    nk: NamedKey,
    locale: &str,
    style: LabelStyle,
    localizer: &L,
) -> Cow<'static, str> {
    match localizer::translate_for(nk.key_id(), locale, style, localizer) {
        Some(s) => s,
        None => Cow::Borrowed(nk.key_id()),
    }
}

/// Reverse lookup — find the scancode whose *base-level* keycode matches.
///
/// Useful for cross-layout bridging: `scancode_for(k, "windows/ru-t-k0-jcuken")`
/// then `resolve(sc, NONE, "windows/fr-t-k0-azerty", …)` to re-render a
/// Russian-layout binding on a French-layout keyboard.
pub fn scancode_for(keycode: Keycode, layout: &str) -> Option<Scancode> {
    let l = layout::get_layout(layout)?;
    for k in l.printable_keys.iter() {
        if layout::layout_key_base_keycode(k) == keycode {
            return Some(k.scancode);
        }
    }
    for k in l.named_keys.iter() {
        if layout::layout_key_base_keycode(k) == keycode {
            return Some(k.scancode);
        }
    }
    None
}

/// Localize the label for a single held modifier — platform-aware.
///
/// ```text
/// modifier_label(Gui, Mac,   "en", Symbolic, …) == "⌘"
/// modifier_label(Gui, Win,   "fr", Textual,  …) == "Windows"
/// modifier_label(Alt, Mac,   "fr", Textual,  …) == "Option"
/// modifier_label(Gui, Linux, "en", Textual,  …) == "Super"
/// ```
pub fn modifier_label<L: KeyLocalizer>(
    modifier: Modifier,
    platform: Platform,
    locale: &str,
    style: LabelStyle,
    localizer: &L,
) -> Cow<'static, str> {
    let key = format!("{}_{}", modifier.key_id_prefix(), platform.id());
    if let Some(s) = localizer::translate_for(&key, locale, style, localizer) {
        return s;
    }
    let generic = modifier.key_id_prefix();
    localizer::translate_for(generic, locale, style, localizer).unwrap_or(Cow::Borrowed(generic))
}

/// Inverse of `SDL_GetKeyName` — parse a textual key name into its
/// canonical [`Keycode`].
///
/// Accepts the names SDL emits (`"Escape"`, `"Left Shift"`, `"F5"`,
/// `"a"`, `"1"`, …). Matching is case-insensitive for letters and
/// the `"Keypad X"` prefix; other named keys are matched verbatim.
pub fn keycode_from_name(name: &str) -> Option<Keycode> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Single-character printable names map to their Unicode code point.
    let mut chars = trimmed.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        if !c.is_control() {
            let lower = c.to_lowercase().next().unwrap_or(c);
            return Some(Keycode::from(lower));
        }
    }

    // Case-insensitive match against a fixed table.
    let upper = trimmed.to_ascii_uppercase();
    match upper.as_str() {
        "RETURN" | "ENTER" => Some(Keycode::RETURN),
        "ESCAPE" | "ESC" => Some(Keycode::ESCAPE),
        "BACKSPACE" => Some(Keycode::BACKSPACE),
        "TAB" => Some(Keycode::TAB),
        "SPACE" => Some(Keycode::SPACE),
        "CAPSLOCK" | "CAPS LOCK" => Some(Keycode::CAPSLOCK),
        "NUMLOCK" | "NUM LOCK" | "NUMLOCKCLEAR" => Some(Keycode::NUM_LOCK_CLEAR),
        "SCROLLLOCK" | "SCROLL LOCK" => Some(Keycode::SCROLL_LOCK),
        "PRINTSCREEN" | "PRINT SCREEN" => Some(Keycode::PRINT_SCREEN),
        "PAUSE" => Some(Keycode::PAUSE),
        "INSERT" => Some(Keycode::INSERT),
        "HOME" => Some(Keycode::HOME),
        "PAGEUP" | "PAGE UP" => Some(Keycode::PAGE_UP),
        "DELETE" => Some(Keycode::DELETE),
        "END" => Some(Keycode::END),
        "PAGEDOWN" | "PAGE DOWN" => Some(Keycode::PAGE_DOWN),
        "RIGHT" => Some(Keycode::RIGHT),
        "LEFT" => Some(Keycode::LEFT),
        "DOWN" => Some(Keycode::DOWN),
        "UP" => Some(Keycode::UP),
        "APPLICATION" => Some(Keycode::APPLICATION),
        "MENU" => Some(Keycode::MENU),
        "LEFT CTRL" | "LCTRL" => Some(Keycode::LCTRL),
        "RIGHT CTRL" | "RCTRL" => Some(Keycode::RCTRL),
        "LEFT SHIFT" | "LSHIFT" => Some(Keycode::LSHIFT),
        "RIGHT SHIFT" | "RSHIFT" => Some(Keycode::RSHIFT),
        "LEFT ALT" | "LALT" => Some(Keycode::LALT),
        "RIGHT ALT" | "RALT" | "ALTGR" => Some(Keycode::RALT),
        "LEFT GUI" | "LGUI" => Some(Keycode::LGUI),
        "RIGHT GUI" | "RGUI" => Some(Keycode::RGUI),
        "F1" => Some(Keycode::F1),
        "F2" => Some(Keycode::F2),
        "F3" => Some(Keycode::F3),
        "F4" => Some(Keycode::F4),
        "F5" => Some(Keycode::F5),
        "F6" => Some(Keycode::F6),
        "F7" => Some(Keycode::F7),
        "F8" => Some(Keycode::F8),
        "F9" => Some(Keycode::F9),
        "F10" => Some(Keycode::F10),
        "F11" => Some(Keycode::F11),
        "F12" => Some(Keycode::F12),
        "F13" => Some(Keycode::F13),
        "F14" => Some(Keycode::F14),
        "F15" => Some(Keycode::F15),
        "F16" => Some(Keycode::F16),
        "F17" => Some(Keycode::F17),
        "F18" => Some(Keycode::F18),
        "F19" => Some(Keycode::F19),
        "F20" => Some(Keycode::F20),
        "F21" => Some(Keycode::F21),
        "F22" => Some(Keycode::F22),
        "F23" => Some(Keycode::F23),
        "F24" => Some(Keycode::F24),
        "KEYPAD 0" | "KP_0" => Some(Keycode::KP_0),
        "KEYPAD 1" | "KP_1" => Some(Keycode::KP_1),
        "KEYPAD 2" | "KP_2" => Some(Keycode::KP_2),
        "KEYPAD 3" | "KP_3" => Some(Keycode::KP_3),
        "KEYPAD 4" | "KP_4" => Some(Keycode::KP_4),
        "KEYPAD 5" | "KP_5" => Some(Keycode::KP_5),
        "KEYPAD 6" | "KP_6" => Some(Keycode::KP_6),
        "KEYPAD 7" | "KP_7" => Some(Keycode::KP_7),
        "KEYPAD 8" | "KP_8" => Some(Keycode::KP_8),
        "KEYPAD 9" | "KP_9" => Some(Keycode::KP_9),
        "KEYPAD ." | "KP_PERIOD" => Some(Keycode::KP_PERIOD),
        "KEYPAD =" | "KP_EQUALS" => Some(Keycode::KP_EQUALS),
        "KEYPAD ENTER" | "KP_ENTER" => Some(Keycode::KP_ENTER),
        "KEYPAD /" | "KP_DIVIDE" => Some(Keycode::KP_DIVIDE),
        "KEYPAD *" | "KP_MULTIPLY" => Some(Keycode::KP_MULTIPLY),
        "KEYPAD -" | "KP_MINUS" => Some(Keycode::KP_MINUS),
        "KEYPAD +" | "KP_PLUS" => Some(Keycode::KP_PLUS),
        _ => None,
    }
}
