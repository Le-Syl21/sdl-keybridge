//! Keyboard layouts.
//!
//! A [`Layout`] maps physical [`Scancode`]s to the glyphs they produce at
//! each modifier level (Base / Shift / AltGr / Shift+AltGr), plus the
//! [`NamedKey`] identity for non-printable keys.
//!
//! The layouts shipped here are hand-curated for v0.1. The long-term
//! plan is to generate them at build time from the Unicode CLDR
//! keyboard data (see `PLAN.md` and `CONTRIBUTING.md`).

use crate::keycode::Keycode;
use crate::localizer::Platform;
use crate::named_key::NamedKey;
use crate::scancode::Scancode;

/// One physical key within a [`Layout`].
#[derive(Copy, Clone, Debug)]
pub struct LayoutKey {
    pub scancode: Scancode,
    /// Glyph produced at the base (unshifted) level. `None` means the key
    /// has no printable output at that level.
    pub base: Option<char>,
    /// Glyph produced with Shift held.
    pub shift: Option<char>,
    /// Glyph produced with AltGr (`KMOD_MODE` or `RALT`) held.
    pub altgr: Option<char>,
    /// Glyph produced with Shift + AltGr held.
    pub shift_altgr: Option<char>,
    /// Named-key identity for keys that are not character-producing
    /// (Escape, arrows, F-keys, …). `None` means the key is printable.
    pub named: Option<NamedKey>,
}

impl LayoutKey {
    /// Short helper — build a pure-named-key entry (no glyphs).
    pub const fn named(sc: Scancode, nk: NamedKey) -> Self {
        Self {
            scancode: sc,
            base: None,
            shift: None,
            altgr: None,
            shift_altgr: None,
            named: Some(nk),
        }
    }

    /// Build a printable key entry with base + shift glyphs only.
    pub const fn printable(sc: Scancode, base: char, shift: char) -> Self {
        Self {
            scancode: sc,
            base: Some(base),
            shift: Some(shift),
            altgr: None,
            shift_altgr: None,
            named: None,
        }
    }

    /// Build a printable key entry with all 4 modifier levels.
    pub const fn printable4(
        sc: Scancode,
        base: char,
        shift: char,
        altgr: char,
        shift_altgr: char,
    ) -> Self {
        Self {
            scancode: sc,
            base: Some(base),
            shift: Some(shift),
            altgr: Some(altgr),
            shift_altgr: Some(shift_altgr),
            named: None,
        }
    }
}

/// A complete keyboard layout.
#[derive(Copy, Clone, Debug)]
pub struct Layout {
    /// Layout identifier — `"<platform>/<BCP 47>-t-k0-<variant>"`.
    pub id: &'static str,
    /// Display name suitable for UIs (e.g. `"French (AZERTY)"`).
    pub display_name: &'static str,
    /// Host platform this layout was designed for.
    pub platform: Platform,
    /// Primary language subtag — helps match a UI locale to this layout.
    pub language: &'static str,
    /// Non-printable named keys — usually the `STD_NAMED_KEYS` constant.
    pub named_keys: &'static [LayoutKey],
    /// Character-producing keys — layout-specific.
    pub printable_keys: &'static [LayoutKey],
}

impl Layout {
    /// Find a key by scancode.
    pub fn key(&self, sc: Scancode) -> Option<&'static LayoutKey> {
        self.printable_keys
            .iter()
            .find(|k| k.scancode == sc)
            .or_else(|| self.named_keys.iter().find(|k| k.scancode == sc))
    }
}

/// Named (non-printable) keys shared by every layout — their identity is
/// defined by the scancode, not by the layout.
pub const STD_NAMED_KEYS: &[LayoutKey] = &[
    LayoutKey::named(Scancode::ESCAPE, NamedKey::Escape),
    LayoutKey::named(Scancode::RETURN, NamedKey::Return),
    LayoutKey::named(Scancode::BACKSPACE, NamedKey::Backspace),
    LayoutKey::named(Scancode::TAB, NamedKey::Tab),
    LayoutKey::named(Scancode::CAPSLOCK, NamedKey::CapsLock),
    LayoutKey::named(Scancode::F1, NamedKey::F1),
    LayoutKey::named(Scancode::F2, NamedKey::F2),
    LayoutKey::named(Scancode::F3, NamedKey::F3),
    LayoutKey::named(Scancode::F4, NamedKey::F4),
    LayoutKey::named(Scancode::F5, NamedKey::F5),
    LayoutKey::named(Scancode::F6, NamedKey::F6),
    LayoutKey::named(Scancode::F7, NamedKey::F7),
    LayoutKey::named(Scancode::F8, NamedKey::F8),
    LayoutKey::named(Scancode::F9, NamedKey::F9),
    LayoutKey::named(Scancode::F10, NamedKey::F10),
    LayoutKey::named(Scancode::F11, NamedKey::F11),
    LayoutKey::named(Scancode::F12, NamedKey::F12),
    LayoutKey::named(Scancode::F13, NamedKey::F13),
    LayoutKey::named(Scancode::F14, NamedKey::F14),
    LayoutKey::named(Scancode::F15, NamedKey::F15),
    LayoutKey::named(Scancode::F16, NamedKey::F16),
    LayoutKey::named(Scancode::F17, NamedKey::F17),
    LayoutKey::named(Scancode::F18, NamedKey::F18),
    LayoutKey::named(Scancode::F19, NamedKey::F19),
    LayoutKey::named(Scancode::F20, NamedKey::F20),
    LayoutKey::named(Scancode::F21, NamedKey::F21),
    LayoutKey::named(Scancode::F22, NamedKey::F22),
    LayoutKey::named(Scancode::F23, NamedKey::F23),
    LayoutKey::named(Scancode::F24, NamedKey::F24),
    LayoutKey::named(Scancode::PRINT_SCREEN, NamedKey::PrintScreen),
    LayoutKey::named(Scancode::SCROLL_LOCK, NamedKey::ScrollLock),
    LayoutKey::named(Scancode::PAUSE, NamedKey::Pause),
    LayoutKey::named(Scancode::INSERT, NamedKey::Insert),
    LayoutKey::named(Scancode::HOME, NamedKey::Home),
    LayoutKey::named(Scancode::PAGE_UP, NamedKey::PageUp),
    LayoutKey::named(Scancode::DELETE, NamedKey::Delete),
    LayoutKey::named(Scancode::END, NamedKey::End),
    LayoutKey::named(Scancode::PAGE_DOWN, NamedKey::PageDown),
    LayoutKey::named(Scancode::RIGHT, NamedKey::ArrowRight),
    LayoutKey::named(Scancode::LEFT, NamedKey::ArrowLeft),
    LayoutKey::named(Scancode::DOWN, NamedKey::ArrowDown),
    LayoutKey::named(Scancode::UP, NamedKey::ArrowUp),
    LayoutKey::named(Scancode::NUM_LOCK_CLEAR, NamedKey::NumLock),
    LayoutKey::named(Scancode::KP_DIVIDE, NamedKey::KeypadDivide),
    LayoutKey::named(Scancode::KP_MULTIPLY, NamedKey::KeypadMultiply),
    LayoutKey::named(Scancode::KP_MINUS, NamedKey::KeypadMinus),
    LayoutKey::named(Scancode::KP_PLUS, NamedKey::KeypadPlus),
    LayoutKey::named(Scancode::KP_ENTER, NamedKey::KeypadEnter),
    LayoutKey::named(Scancode::KP_1, NamedKey::Keypad1),
    LayoutKey::named(Scancode::KP_2, NamedKey::Keypad2),
    LayoutKey::named(Scancode::KP_3, NamedKey::Keypad3),
    LayoutKey::named(Scancode::KP_4, NamedKey::Keypad4),
    LayoutKey::named(Scancode::KP_5, NamedKey::Keypad5),
    LayoutKey::named(Scancode::KP_6, NamedKey::Keypad6),
    LayoutKey::named(Scancode::KP_7, NamedKey::Keypad7),
    LayoutKey::named(Scancode::KP_8, NamedKey::Keypad8),
    LayoutKey::named(Scancode::KP_9, NamedKey::Keypad9),
    LayoutKey::named(Scancode::KP_0, NamedKey::Keypad0),
    LayoutKey::named(Scancode::KP_PERIOD, NamedKey::KeypadPeriod),
    LayoutKey::named(Scancode::KP_EQUALS, NamedKey::KeypadEquals),
    LayoutKey::named(Scancode::APPLICATION, NamedKey::Application),
    LayoutKey::named(Scancode::MENU, NamedKey::Menu),
    LayoutKey::named(Scancode::SPACE, NamedKey::Space),
    LayoutKey::named(Scancode::LCTRL, NamedKey::ControlLeft),
    LayoutKey::named(Scancode::LSHIFT, NamedKey::ShiftLeft),
    LayoutKey::named(Scancode::LALT, NamedKey::AltLeft),
    LayoutKey::named(Scancode::LGUI, NamedKey::GuiLeft),
    LayoutKey::named(Scancode::RCTRL, NamedKey::ControlRight),
    LayoutKey::named(Scancode::RSHIFT, NamedKey::ShiftRight),
    LayoutKey::named(Scancode::RALT, NamedKey::AltRight),
    LayoutKey::named(Scancode::RGUI, NamedKey::GuiRight),
];

// ==========================================================================
// Layout registry — populated entirely from CLDR 43 via build.rs.
//
// `build.rs` reads every XML under `data/cldr-43/keyboards/` and writes
// `$OUT_DIR/cldr_layouts.rs` containing:
//
// - `CLDR_KEYS_<SYM>`   — one `&[LayoutKey]` per layout
// - `CLDR_LAYOUT_<SYM>` — one `Layout` per layout
// - `CLDR_LAYOUTS`      — the flat `&[&Layout]` registry.
//
// The crate has no hand-authored layout data — every glyph served by
// `resolve()` / `scancode_for()` comes from a CLDR-maintained XML.
// ==========================================================================

include!(concat!(env!("OUT_DIR"), "/cldr_layouts.rs"));

/// Look up a layout by its id — either a CLDR 2.x tag like
/// `"windows/fr-t-k0-windows"` or a CLDR 3.0 tag like `"cldr3/fr"`.
pub fn get_layout(id: &str) -> Option<&'static Layout> {
    CLDR_LAYOUTS
        .iter()
        .find(|l| l.id == id)
        .or_else(|| CLDR3_LAYOUTS.iter().find(|l| l.id == id))
        .copied()
}

/// Every layout shipped in this build — concatenates the CLDR 2.x and
/// CLDR 3.0 registries. The `Vec` is allocated once on first call and
/// returned as a borrowed slice thereafter.
pub fn all_layouts() -> &'static [&'static Layout] {
    use std::sync::OnceLock;
    static COMBINED: OnceLock<Vec<&'static Layout>> = OnceLock::new();
    COMBINED
        .get_or_init(|| {
            let mut v: Vec<&'static Layout> = CLDR_LAYOUTS.to_vec();
            v.extend(CLDR3_LAYOUTS.iter().copied());
            v
        })
        .as_slice()
}

/// Compute the canonical SDL keycode for a [`LayoutKey`] at its base
/// modifier level.
pub(crate) fn layout_key_base_keycode(k: &LayoutKey) -> Keycode {
    if let Some(nk) = k.named {
        named_key_keycode(nk)
    } else if let Some(c) = k.base {
        Keycode::from(c)
    } else {
        Keycode::UNKNOWN
    }
}

/// The canonical SDL keycode for a [`NamedKey`].
pub(crate) fn named_key_keycode(nk: NamedKey) -> Keycode {
    use NamedKey::*;
    match nk {
        Escape => Keycode::ESCAPE,
        Return => Keycode::RETURN,
        Tab => Keycode::TAB,
        Space => Keycode::SPACE,
        Backspace => Keycode::BACKSPACE,
        Insert => Keycode::INSERT,
        Delete => Keycode::DELETE,
        Home => Keycode::HOME,
        End => Keycode::END,
        PageUp => Keycode::PAGE_UP,
        PageDown => Keycode::PAGE_DOWN,
        ArrowUp => Keycode::UP,
        ArrowDown => Keycode::DOWN,
        ArrowLeft => Keycode::LEFT,
        ArrowRight => Keycode::RIGHT,
        CapsLock => Keycode::CAPSLOCK,
        NumLock => Keycode::NUM_LOCK_CLEAR,
        ScrollLock => Keycode::SCROLL_LOCK,
        PrintScreen => Keycode::PRINT_SCREEN,
        Pause => Keycode::PAUSE,
        Menu => Keycode::MENU,
        Application => Keycode::APPLICATION,
        ShiftLeft => Keycode::LSHIFT,
        ShiftRight => Keycode::RSHIFT,
        ControlLeft => Keycode::LCTRL,
        ControlRight => Keycode::RCTRL,
        AltLeft => Keycode::LALT,
        AltRight => Keycode::RALT,
        AltGr => Keycode::RALT,
        GuiLeft => Keycode::LGUI,
        GuiRight => Keycode::RGUI,
        F1 => Keycode::F1,
        F2 => Keycode::F2,
        F3 => Keycode::F3,
        F4 => Keycode::F4,
        F5 => Keycode::F5,
        F6 => Keycode::F6,
        F7 => Keycode::F7,
        F8 => Keycode::F8,
        F9 => Keycode::F9,
        F10 => Keycode::F10,
        F11 => Keycode::F11,
        F12 => Keycode::F12,
        F13 => Keycode::F13,
        F14 => Keycode::F14,
        F15 => Keycode::F15,
        F16 => Keycode::F16,
        F17 => Keycode::F17,
        F18 => Keycode::F18,
        F19 => Keycode::F19,
        F20 => Keycode::F20,
        F21 => Keycode::F21,
        F22 => Keycode::F22,
        F23 => Keycode::F23,
        F24 => Keycode::F24,
        KeypadEnter => Keycode::KP_ENTER,
        KeypadDivide => Keycode::KP_DIVIDE,
        KeypadMultiply => Keycode::KP_MULTIPLY,
        KeypadMinus => Keycode::KP_MINUS,
        KeypadPlus => Keycode::KP_PLUS,
        KeypadPeriod => Keycode::KP_PERIOD,
        KeypadEquals => Keycode::KP_EQUALS,
        Keypad0 => Keycode::KP_0,
        Keypad1 => Keycode::KP_1,
        Keypad2 => Keycode::KP_2,
        Keypad3 => Keycode::KP_3,
        Keypad4 => Keycode::KP_4,
        Keypad5 => Keycode::KP_5,
        Keypad6 => Keycode::KP_6,
        Keypad7 => Keycode::KP_7,
        Keypad8 => Keycode::KP_8,
        Keypad9 => Keycode::KP_9,
    }
}
