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
// US QWERTY
// ==========================================================================

pub(crate) const QWERTY_US_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '@'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '^'),
    LayoutKey::printable(Scancode::NUM_7, '7', '&'),
    LayoutKey::printable(Scancode::NUM_8, '8', '*'),
    LayoutKey::printable(Scancode::NUM_9, '9', '('),
    LayoutKey::printable(Scancode::NUM_0, '0', ')'),
    LayoutKey::printable(Scancode::MINUS, '-', '_'),
    LayoutKey::printable(Scancode::EQUALS, '=', '+'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '[', '{'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, ']', '}'),
    LayoutKey::printable(Scancode::BACKSLASH, '\\', '|'),
    LayoutKey::printable(Scancode::SEMICOLON, ';', ':'),
    LayoutKey::printable(Scancode::APOSTROPHE, '\'', '"'),
    LayoutKey::printable(Scancode::GRAVE, '`', '~'),
    LayoutKey::printable(Scancode::COMMA, ',', '<'),
    LayoutKey::printable(Scancode::PERIOD, '.', '>'),
    LayoutKey::printable(Scancode::SLASH, '/', '?'),
];

pub const LAYOUT_MAC_EN_US_QWERTY: Layout = Layout {
    id: "mac/en-US-t-k0-qwerty",
    display_name: "English — QWERTY (US)",
    platform: Platform::Mac,
    language: "en",
    named_keys: STD_NAMED_KEYS,
    printable_keys: QWERTY_US_KEYS,
};

pub const LAYOUT_WIN_EN_US_QWERTY: Layout = Layout {
    id: "windows/en-US-t-k0-qwerty",
    display_name: "English — QWERTY (US)",
    platform: Platform::Windows,
    language: "en",
    named_keys: STD_NAMED_KEYS,
    printable_keys: QWERTY_US_KEYS,
};

pub const LAYOUT_LINUX_EN_US_QWERTY: Layout = Layout {
    id: "linux/en-US-t-k0-qwerty",
    display_name: "English — QWERTY (US)",
    platform: Platform::Linux,
    language: "en",
    named_keys: STD_NAMED_KEYS,
    printable_keys: QWERTY_US_KEYS,
};

// ==========================================================================
// French AZERTY (France, ISO variant)
// ==========================================================================

pub(crate) const AZERTY_FR_KEYS: &[LayoutKey] = &[
    // Letters — Q/W and A/Z are swapped vs QWERTY; M is on the right of L.
    LayoutKey::printable(Scancode::Q, 'a', 'A'),
    LayoutKey::printable(Scancode::W, 'z', 'Z'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::A, 'q', 'Q'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::SEMICOLON, 'm', 'M'),
    LayoutKey::printable(Scancode::Z, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::M, ',', '?'),
    LayoutKey::printable(Scancode::COMMA, ';', '.'),
    LayoutKey::printable(Scancode::PERIOD, ':', '/'),
    LayoutKey::printable(Scancode::SLASH, '!', '§'),
    // Digit row — SHIFT gives digits (because AZERTY has symbols unshifted).
    LayoutKey::printable4(Scancode::NUM_1, '&', '1', '\0', '\0'),
    LayoutKey::printable4(Scancode::NUM_2, 'é', '2', '~', '\0'),
    LayoutKey::printable4(Scancode::NUM_3, '"', '3', '#', '\0'),
    LayoutKey::printable4(Scancode::NUM_4, '\'', '4', '{', '\0'),
    LayoutKey::printable4(Scancode::NUM_5, '(', '5', '[', '\0'),
    LayoutKey::printable4(Scancode::NUM_6, '-', '6', '|', '\0'),
    LayoutKey::printable4(Scancode::NUM_7, 'è', '7', '`', '\0'),
    LayoutKey::printable4(Scancode::NUM_8, '_', '8', '\\', '\0'),
    LayoutKey::printable4(Scancode::NUM_9, 'ç', '9', '^', '\0'),
    LayoutKey::printable4(Scancode::NUM_0, 'à', '0', '@', '\0'),
    LayoutKey::printable4(Scancode::MINUS, ')', '°', ']', '\0'),
    LayoutKey::printable(Scancode::EQUALS, '=', '+'),
    // The '²' key on the grave position (ANSI's `/~`).
    LayoutKey::printable(Scancode::GRAVE, '²', '\0'),
    // Dead keys (we still expose the literal glyph per PLAN's "no dead keys").
    LayoutKey::printable(Scancode::LEFT_BRACKET, '^', '¨'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '$', '£'),
    LayoutKey::printable(Scancode::BACKSLASH, '*', 'µ'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'ù', '%'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
];

pub const LAYOUT_MAC_FR_AZERTY: Layout = Layout {
    id: "mac/fr-t-k0-azerty",
    display_name: "Français — AZERTY",
    platform: Platform::Mac,
    language: "fr",
    named_keys: STD_NAMED_KEYS,
    printable_keys: AZERTY_FR_KEYS,
};

pub const LAYOUT_WIN_FR_AZERTY: Layout = Layout {
    id: "windows/fr-t-k0-azerty",
    display_name: "Français — AZERTY",
    platform: Platform::Windows,
    language: "fr",
    named_keys: STD_NAMED_KEYS,
    printable_keys: AZERTY_FR_KEYS,
};

pub const LAYOUT_LINUX_FR_AZERTY: Layout = Layout {
    id: "linux/fr-t-k0-azerty",
    display_name: "Français — AZERTY",
    platform: Platform::Linux,
    language: "fr",
    named_keys: STD_NAMED_KEYS,
    printable_keys: AZERTY_FR_KEYS,
};

// ==========================================================================
// German QWERTZ
// ==========================================================================

pub(crate) const QWERTZ_DE_KEYS: &[LayoutKey] = &[
    // Y/Z swap vs QWERTY.
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::Z, 'y', 'Y'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::Y, 'z', 'Z'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    // Umlauts occupy QWERTY punctuation positions.
    LayoutKey::printable(Scancode::SEMICOLON, 'ö', 'Ö'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'ä', 'Ä'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, 'ü', 'Ü'),
    LayoutKey::printable(Scancode::MINUS, 'ß', '?'),
    LayoutKey::printable4(Scancode::RIGHT_BRACKET, '+', '*', '~', '\0'),
    LayoutKey::printable4(Scancode::BACKSLASH, '#', '\'', '\0', '\0'),
    LayoutKey::printable(Scancode::GRAVE, '^', '°'),
    LayoutKey::printable(Scancode::COMMA, ',', ';'),
    LayoutKey::printable(Scancode::PERIOD, '.', ':'),
    LayoutKey::printable(Scancode::SLASH, '-', '_'),
    LayoutKey::printable(Scancode::EQUALS, '´', '`'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
    LayoutKey::printable4(Scancode::NUM_1, '1', '!', '\0', '\0'),
    LayoutKey::printable4(Scancode::NUM_2, '2', '"', '²', '\0'),
    LayoutKey::printable4(Scancode::NUM_3, '3', '§', '³', '\0'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable4(Scancode::NUM_7, '7', '/', '{', '\0'),
    LayoutKey::printable4(Scancode::NUM_8, '8', '(', '[', '\0'),
    LayoutKey::printable4(Scancode::NUM_9, '9', ')', ']', '\0'),
    LayoutKey::printable4(Scancode::NUM_0, '0', '=', '}', '\0'),
];

pub const LAYOUT_MAC_DE_QWERTZ: Layout = Layout {
    id: "mac/de-t-k0-qwertz",
    display_name: "Deutsch — QWERTZ",
    platform: Platform::Mac,
    language: "de",
    named_keys: STD_NAMED_KEYS,
    printable_keys: QWERTZ_DE_KEYS,
};

pub const LAYOUT_WIN_DE_QWERTZ: Layout = Layout {
    id: "windows/de-t-k0-qwertz",
    display_name: "Deutsch — QWERTZ",
    platform: Platform::Windows,
    language: "de",
    named_keys: STD_NAMED_KEYS,
    printable_keys: QWERTZ_DE_KEYS,
};

pub const LAYOUT_LINUX_DE_QWERTZ: Layout = Layout {
    id: "linux/de-t-k0-qwertz",
    display_name: "Deutsch — QWERTZ",
    platform: Platform::Linux,
    language: "de",
    named_keys: STD_NAMED_KEYS,
    printable_keys: QWERTZ_DE_KEYS,
};

// ==========================================================================
// Russian JCUKEN (Й-Ц-У-К-Е-Н)
// ==========================================================================

pub(crate) const JCUKEN_RU_KEYS: &[LayoutKey] = &[
    // Row 1: Й Ц У К Е Н Г Ш Щ З Х Ъ
    LayoutKey::printable(Scancode::Q, 'й', 'Й'),
    LayoutKey::printable(Scancode::W, 'ц', 'Ц'),
    LayoutKey::printable(Scancode::E, 'у', 'У'),
    LayoutKey::printable(Scancode::R, 'к', 'К'),
    LayoutKey::printable(Scancode::T, 'е', 'Е'),
    LayoutKey::printable(Scancode::Y, 'н', 'Н'),
    LayoutKey::printable(Scancode::U, 'г', 'Г'),
    LayoutKey::printable(Scancode::I, 'ш', 'Ш'),
    LayoutKey::printable(Scancode::O, 'щ', 'Щ'),
    LayoutKey::printable(Scancode::P, 'з', 'З'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, 'х', 'Х'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, 'ъ', 'Ъ'),
    // Row 2: Ф Ы В А П Р О Л Д Ж Э
    LayoutKey::printable(Scancode::A, 'ф', 'Ф'),
    LayoutKey::printable(Scancode::S, 'ы', 'Ы'),
    LayoutKey::printable(Scancode::D, 'в', 'В'),
    LayoutKey::printable(Scancode::F, 'а', 'А'),
    LayoutKey::printable(Scancode::G, 'п', 'П'),
    LayoutKey::printable(Scancode::H, 'р', 'Р'),
    LayoutKey::printable(Scancode::J, 'о', 'О'),
    LayoutKey::printable(Scancode::K, 'л', 'Л'),
    LayoutKey::printable(Scancode::L, 'д', 'Д'),
    LayoutKey::printable(Scancode::SEMICOLON, 'ж', 'Ж'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'э', 'Э'),
    // Row 3: Я Ч С М И Т Ь Б Ю .
    LayoutKey::printable(Scancode::Z, 'я', 'Я'),
    LayoutKey::printable(Scancode::X, 'ч', 'Ч'),
    LayoutKey::printable(Scancode::C, 'с', 'С'),
    LayoutKey::printable(Scancode::V, 'м', 'М'),
    LayoutKey::printable(Scancode::B, 'и', 'И'),
    LayoutKey::printable(Scancode::N, 'т', 'Т'),
    LayoutKey::printable(Scancode::M, 'ь', 'Ь'),
    LayoutKey::printable(Scancode::COMMA, 'б', 'Б'),
    LayoutKey::printable(Scancode::PERIOD, 'ю', 'Ю'),
    LayoutKey::printable(Scancode::SLASH, '.', ','),
    // Digit row (same digits, different punctuation).
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '№'),
    LayoutKey::printable(Scancode::NUM_4, '4', ';'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', ':'),
    LayoutKey::printable(Scancode::NUM_7, '7', '?'),
    LayoutKey::printable(Scancode::NUM_8, '8', '*'),
    LayoutKey::printable(Scancode::NUM_9, '9', '('),
    LayoutKey::printable(Scancode::NUM_0, '0', ')'),
    LayoutKey::printable(Scancode::MINUS, '-', '_'),
    LayoutKey::printable(Scancode::EQUALS, '=', '+'),
    LayoutKey::printable(Scancode::GRAVE, 'ё', 'Ё'),
    LayoutKey::printable(Scancode::BACKSLASH, '\\', '/'),
];

pub const LAYOUT_WIN_RU_JCUKEN: Layout = Layout {
    id: "windows/ru-t-k0-jcuken",
    display_name: "Русский — ЙЦУКЕН",
    platform: Platform::Windows,
    language: "ru",
    named_keys: STD_NAMED_KEYS,
    printable_keys: JCUKEN_RU_KEYS,
};

pub const LAYOUT_LINUX_RU_JCUKEN: Layout = Layout {
    id: "linux/ru-t-k0-jcuken",
    display_name: "Русский — ЙЦУКЕН",
    platform: Platform::Linux,
    language: "ru",
    named_keys: STD_NAMED_KEYS,
    printable_keys: JCUKEN_RU_KEYS,
};

pub const LAYOUT_MAC_RU_JCUKEN: Layout = Layout {
    id: "mac/ru-t-k0-jcuken",
    display_name: "Русский — ЙЦУКЕН",
    platform: Platform::Mac,
    language: "ru",
    named_keys: STD_NAMED_KEYS,
    printable_keys: JCUKEN_RU_KEYS,
};

// ==========================================================================
// US Dvorak
// ==========================================================================

pub(crate) const DVORAK_EN_US_KEYS: &[LayoutKey] = &[
    // Top letter row: ' , . P Y F G C R L
    LayoutKey::printable(Scancode::Q, '\'', '"'),
    LayoutKey::printable(Scancode::W, ',', '<'),
    LayoutKey::printable(Scancode::E, '.', '>'),
    LayoutKey::printable(Scancode::R, 'p', 'P'),
    LayoutKey::printable(Scancode::T, 'y', 'Y'),
    LayoutKey::printable(Scancode::Y, 'f', 'F'),
    LayoutKey::printable(Scancode::U, 'g', 'G'),
    LayoutKey::printable(Scancode::I, 'c', 'C'),
    LayoutKey::printable(Scancode::O, 'r', 'R'),
    LayoutKey::printable(Scancode::P, 'l', 'L'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '/', '?'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '=', '+'),
    // Middle row: A O E U I D H T N S -
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::S, 'o', 'O'),
    LayoutKey::printable(Scancode::D, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'u', 'U'),
    LayoutKey::printable(Scancode::G, 'i', 'I'),
    LayoutKey::printable(Scancode::H, 'd', 'D'),
    LayoutKey::printable(Scancode::J, 'h', 'H'),
    LayoutKey::printable(Scancode::K, 't', 'T'),
    LayoutKey::printable(Scancode::L, 'n', 'N'),
    LayoutKey::printable(Scancode::SEMICOLON, 's', 'S'),
    LayoutKey::printable(Scancode::APOSTROPHE, '-', '_'),
    // Bottom row: ; Q J K X B M W V Z
    LayoutKey::printable(Scancode::Z, ';', ':'),
    LayoutKey::printable(Scancode::X, 'q', 'Q'),
    LayoutKey::printable(Scancode::C, 'j', 'J'),
    LayoutKey::printable(Scancode::V, 'k', 'K'),
    LayoutKey::printable(Scancode::B, 'x', 'X'),
    LayoutKey::printable(Scancode::N, 'b', 'B'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::COMMA, 'w', 'W'),
    LayoutKey::printable(Scancode::PERIOD, 'v', 'V'),
    LayoutKey::printable(Scancode::SLASH, 'z', 'Z'),
    // Digits — identical to QWERTY US.
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '@'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '^'),
    LayoutKey::printable(Scancode::NUM_7, '7', '&'),
    LayoutKey::printable(Scancode::NUM_8, '8', '*'),
    LayoutKey::printable(Scancode::NUM_9, '9', '('),
    LayoutKey::printable(Scancode::NUM_0, '0', ')'),
    LayoutKey::printable(Scancode::MINUS, '[', '{'),
    LayoutKey::printable(Scancode::EQUALS, ']', '}'),
    LayoutKey::printable(Scancode::GRAVE, '`', '~'),
    LayoutKey::printable(Scancode::BACKSLASH, '\\', '|'),
];

pub const LAYOUT_MAC_EN_US_DVORAK: Layout = Layout {
    id: "mac/en-US-t-k0-dvorak",
    display_name: "English — Dvorak (US)",
    platform: Platform::Mac,
    language: "en",
    named_keys: STD_NAMED_KEYS,
    printable_keys: DVORAK_EN_US_KEYS,
};

pub const LAYOUT_WIN_EN_US_DVORAK: Layout = Layout {
    id: "windows/en-US-t-k0-dvorak",
    display_name: "English — Dvorak (US)",
    platform: Platform::Windows,
    language: "en",
    named_keys: STD_NAMED_KEYS,
    printable_keys: DVORAK_EN_US_KEYS,
};

pub const LAYOUT_LINUX_EN_US_DVORAK: Layout = Layout {
    id: "linux/en-US-t-k0-dvorak",
    display_name: "English — Dvorak (US)",
    platform: Platform::Linux,
    language: "en",
    named_keys: STD_NAMED_KEYS,
    printable_keys: DVORAK_EN_US_KEYS,
};

// ==========================================================================
// Helper used by the layouts below to cut boilerplate — one line per
// platform variant instead of a 9-line struct literal.
// ==========================================================================

const fn mk_layout(
    id: &'static str,
    display_name: &'static str,
    platform: Platform,
    language: &'static str,
    keys: &'static [LayoutKey],
) -> Layout {
    Layout {
        id,
        display_name,
        platform,
        language,
        named_keys: STD_NAMED_KEYS,
        printable_keys: keys,
    }
}

// ==========================================================================
// English — UK ISO QWERTY
// ==========================================================================

pub(crate) const QWERTY_EN_GB_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '£'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '^'),
    LayoutKey::printable(Scancode::NUM_7, '7', '&'),
    LayoutKey::printable(Scancode::NUM_8, '8', '*'),
    LayoutKey::printable(Scancode::NUM_9, '9', '('),
    LayoutKey::printable(Scancode::NUM_0, '0', ')'),
    LayoutKey::printable(Scancode::MINUS, '-', '_'),
    LayoutKey::printable(Scancode::EQUALS, '=', '+'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '[', '{'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, ']', '}'),
    LayoutKey::printable(Scancode::BACKSLASH, '#', '~'),
    LayoutKey::printable(Scancode::SEMICOLON, ';', ':'),
    LayoutKey::printable(Scancode::APOSTROPHE, '\'', '@'),
    LayoutKey::printable(Scancode::GRAVE, '`', '¬'),
    LayoutKey::printable(Scancode::COMMA, ',', '<'),
    LayoutKey::printable(Scancode::PERIOD, '.', '>'),
    LayoutKey::printable(Scancode::SLASH, '/', '?'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '\\', '|'),
];

pub const LAYOUT_MAC_EN_GB_QWERTY: Layout = mk_layout(
    "mac/en-GB-t-k0-qwerty",
    "English — QWERTY (UK)",
    Platform::Mac,
    "en",
    QWERTY_EN_GB_KEYS,
);
pub const LAYOUT_WIN_EN_GB_QWERTY: Layout = mk_layout(
    "windows/en-GB-t-k0-qwerty",
    "English — QWERTY (UK)",
    Platform::Windows,
    "en",
    QWERTY_EN_GB_KEYS,
);
pub const LAYOUT_LINUX_EN_GB_QWERTY: Layout = mk_layout(
    "linux/en-GB-t-k0-qwerty",
    "English — QWERTY (UK)",
    Platform::Linux,
    "en",
    QWERTY_EN_GB_KEYS,
);

// ==========================================================================
// English — US International (QWERTY with AltGr accents; dead keys
// reported as their bare glyphs per this crate's no-composition rule).
// ==========================================================================

pub(crate) const QWERTY_EN_US_INTL_KEYS: &[LayoutKey] = &[
    LayoutKey::printable4(Scancode::A, 'a', 'A', 'á', 'Á'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable4(Scancode::C, 'c', 'C', '©', '¢'),
    LayoutKey::printable4(Scancode::D, 'd', 'D', 'ð', 'Ð'),
    LayoutKey::printable4(Scancode::E, 'e', 'E', 'é', 'É'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable4(Scancode::I, 'i', 'I', 'í', 'Í'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable4(Scancode::M, 'm', 'M', 'µ', 'Μ'),
    LayoutKey::printable4(Scancode::N, 'n', 'N', 'ñ', 'Ñ'),
    LayoutKey::printable4(Scancode::O, 'o', 'O', 'ó', 'Ó'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable4(Scancode::Q, 'q', 'Q', 'ä', 'Ä'),
    LayoutKey::printable4(Scancode::R, 'r', 'R', '®', '®'),
    LayoutKey::printable4(Scancode::S, 's', 'S', 'ß', '§'),
    LayoutKey::printable4(Scancode::T, 't', 'T', 'þ', 'Þ'),
    LayoutKey::printable4(Scancode::U, 'u', 'U', 'ú', 'Ú'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable4(Scancode::W, 'w', 'W', 'å', 'Å'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable4(Scancode::Y, 'y', 'Y', 'ü', 'Ü'),
    LayoutKey::printable4(Scancode::Z, 'z', 'Z', 'æ', 'Æ'),
    LayoutKey::printable4(Scancode::NUM_1, '1', '!', '¡', '¹'),
    LayoutKey::printable4(Scancode::NUM_2, '2', '@', '²', '²'),
    LayoutKey::printable4(Scancode::NUM_3, '3', '#', '³', '³'),
    LayoutKey::printable4(Scancode::NUM_4, '4', '$', '¤', '£'),
    LayoutKey::printable4(Scancode::NUM_5, '5', '%', '€', '€'),
    LayoutKey::printable(Scancode::NUM_6, '6', '^'),
    LayoutKey::printable(Scancode::NUM_7, '7', '&'),
    LayoutKey::printable(Scancode::NUM_8, '8', '*'),
    LayoutKey::printable(Scancode::NUM_9, '9', '('),
    LayoutKey::printable(Scancode::NUM_0, '0', ')'),
    LayoutKey::printable4(Scancode::MINUS, '-', '_', '¥', '¥'),
    LayoutKey::printable4(Scancode::EQUALS, '=', '+', '×', '÷'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '[', '{'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, ']', '}'),
    LayoutKey::printable(Scancode::BACKSLASH, '\\', '|'),
    LayoutKey::printable(Scancode::SEMICOLON, ';', ':'),
    LayoutKey::printable(Scancode::APOSTROPHE, '\'', '"'),
    LayoutKey::printable(Scancode::GRAVE, '`', '~'),
    LayoutKey::printable(Scancode::COMMA, ',', '<'),
    LayoutKey::printable(Scancode::PERIOD, '.', '>'),
    LayoutKey::printable4(Scancode::SLASH, '/', '?', '¿', '¿'),
];

pub const LAYOUT_MAC_EN_US_INTL: Layout = mk_layout(
    "mac/en-US-t-k0-intl",
    "English — US International",
    Platform::Mac,
    "en",
    QWERTY_EN_US_INTL_KEYS,
);
pub const LAYOUT_WIN_EN_US_INTL: Layout = mk_layout(
    "windows/en-US-t-k0-intl",
    "English — US International",
    Platform::Windows,
    "en",
    QWERTY_EN_US_INTL_KEYS,
);
pub const LAYOUT_LINUX_EN_US_INTL: Layout = mk_layout(
    "linux/en-US-t-k0-intl",
    "English — US International",
    Platform::Linux,
    "en",
    QWERTY_EN_US_INTL_KEYS,
);

// ==========================================================================
// English — US Colemak
// ==========================================================================

pub(crate) const COLEMAK_EN_US_KEYS: &[LayoutKey] = &[
    // Top letter row: Q W F P G J L U Y ;
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::E, 'f', 'F'),
    LayoutKey::printable(Scancode::R, 'p', 'P'),
    LayoutKey::printable(Scancode::T, 'g', 'G'),
    LayoutKey::printable(Scancode::Y, 'j', 'J'),
    LayoutKey::printable(Scancode::U, 'l', 'L'),
    LayoutKey::printable(Scancode::I, 'u', 'U'),
    LayoutKey::printable(Scancode::O, 'y', 'Y'),
    LayoutKey::printable(Scancode::P, ';', ':'),
    // Home row: A R S T D H N E I O
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::S, 'r', 'R'),
    LayoutKey::printable(Scancode::D, 's', 'S'),
    LayoutKey::printable(Scancode::F, 't', 'T'),
    LayoutKey::printable(Scancode::G, 'd', 'D'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::J, 'n', 'N'),
    LayoutKey::printable(Scancode::K, 'e', 'E'),
    LayoutKey::printable(Scancode::L, 'i', 'I'),
    LayoutKey::printable(Scancode::SEMICOLON, 'o', 'O'),
    LayoutKey::printable(Scancode::APOSTROPHE, '\'', '"'),
    // Bottom row: Z X C V B K M , . /
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::N, 'k', 'K'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::COMMA, ',', '<'),
    LayoutKey::printable(Scancode::PERIOD, '.', '>'),
    LayoutKey::printable(Scancode::SLASH, '/', '?'),
    // Digit row — same as QWERTY-US
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '@'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '^'),
    LayoutKey::printable(Scancode::NUM_7, '7', '&'),
    LayoutKey::printable(Scancode::NUM_8, '8', '*'),
    LayoutKey::printable(Scancode::NUM_9, '9', '('),
    LayoutKey::printable(Scancode::NUM_0, '0', ')'),
    LayoutKey::printable(Scancode::MINUS, '-', '_'),
    LayoutKey::printable(Scancode::EQUALS, '=', '+'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '[', '{'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, ']', '}'),
    LayoutKey::printable(Scancode::BACKSLASH, '\\', '|'),
    LayoutKey::printable(Scancode::GRAVE, '`', '~'),
];

pub const LAYOUT_MAC_EN_US_COLEMAK: Layout = mk_layout(
    "mac/en-US-t-k0-colemak",
    "English — Colemak (US)",
    Platform::Mac,
    "en",
    COLEMAK_EN_US_KEYS,
);
pub const LAYOUT_WIN_EN_US_COLEMAK: Layout = mk_layout(
    "windows/en-US-t-k0-colemak",
    "English — Colemak (US)",
    Platform::Windows,
    "en",
    COLEMAK_EN_US_KEYS,
);
pub const LAYOUT_LINUX_EN_US_COLEMAK: Layout = mk_layout(
    "linux/en-US-t-k0-colemak",
    "English — Colemak (US)",
    Platform::Linux,
    "en",
    COLEMAK_EN_US_KEYS,
);

// ==========================================================================
// Español — Spanish (Spain)
// ==========================================================================

pub(crate) const SPANISH_ES_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    // Ñ occupies the QWERTY ';' position.
    LayoutKey::printable(Scancode::SEMICOLON, 'ñ', 'Ñ'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '·'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable(Scancode::NUM_7, '7', '/'),
    LayoutKey::printable(Scancode::NUM_8, '8', '('),
    LayoutKey::printable(Scancode::NUM_9, '9', ')'),
    LayoutKey::printable(Scancode::NUM_0, '0', '='),
    LayoutKey::printable(Scancode::MINUS, '\'', '?'),
    LayoutKey::printable(Scancode::EQUALS, '¡', '¿'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '`', '^'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '+', '*'),
    LayoutKey::printable(Scancode::BACKSLASH, 'ç', 'Ç'),
    LayoutKey::printable(Scancode::APOSTROPHE, '´', '¨'),
    LayoutKey::printable(Scancode::GRAVE, 'º', 'ª'),
    LayoutKey::printable(Scancode::COMMA, ',', ';'),
    LayoutKey::printable(Scancode::PERIOD, '.', ':'),
    LayoutKey::printable(Scancode::SLASH, '-', '_'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
];

pub const LAYOUT_MAC_ES_SPANISH: Layout = mk_layout(
    "mac/es-t-k0-spanish",
    "Español — Spanish (Spain)",
    Platform::Mac,
    "es",
    SPANISH_ES_KEYS,
);
pub const LAYOUT_WIN_ES_SPANISH: Layout = mk_layout(
    "windows/es-t-k0-spanish",
    "Español — Spanish (Spain)",
    Platform::Windows,
    "es",
    SPANISH_ES_KEYS,
);
pub const LAYOUT_LINUX_ES_SPANISH: Layout = mk_layout(
    "linux/es-t-k0-spanish",
    "Español — Spanish (Spain)",
    Platform::Linux,
    "es",
    SPANISH_ES_KEYS,
);

// ==========================================================================
// Español — Latin American
// ==========================================================================

pub(crate) const LATAM_ES_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::SEMICOLON, 'ñ', 'Ñ'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable(Scancode::NUM_7, '7', '/'),
    LayoutKey::printable(Scancode::NUM_8, '8', '('),
    LayoutKey::printable(Scancode::NUM_9, '9', ')'),
    LayoutKey::printable(Scancode::NUM_0, '0', '='),
    LayoutKey::printable(Scancode::MINUS, '\'', '?'),
    LayoutKey::printable(Scancode::EQUALS, '¿', '¡'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '´', '¨'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '+', '*'),
    LayoutKey::printable(Scancode::BACKSLASH, '}', ']'),
    LayoutKey::printable(Scancode::APOSTROPHE, '{', '['),
    LayoutKey::printable(Scancode::GRAVE, '|', '°'),
    LayoutKey::printable(Scancode::COMMA, ',', ';'),
    LayoutKey::printable(Scancode::PERIOD, '.', ':'),
    LayoutKey::printable(Scancode::SLASH, '-', '_'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
];

pub const LAYOUT_MAC_ES_LATAM: Layout = mk_layout(
    "mac/es-419-t-k0-latin",
    "Español — Latinoamericano",
    Platform::Mac,
    "es",
    LATAM_ES_KEYS,
);
pub const LAYOUT_WIN_ES_LATAM: Layout = mk_layout(
    "windows/es-419-t-k0-latin",
    "Español — Latinoamericano",
    Platform::Windows,
    "es",
    LATAM_ES_KEYS,
);
pub const LAYOUT_LINUX_ES_LATAM: Layout = mk_layout(
    "linux/es-419-t-k0-latin",
    "Español — Latinoamericano",
    Platform::Linux,
    "es",
    LATAM_ES_KEYS,
);

// ==========================================================================
// Italiano — Italian
// ==========================================================================

pub(crate) const ITALIAN_IT_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '£'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable(Scancode::NUM_7, '7', '/'),
    LayoutKey::printable(Scancode::NUM_8, '8', '('),
    LayoutKey::printable(Scancode::NUM_9, '9', ')'),
    LayoutKey::printable(Scancode::NUM_0, '0', '='),
    LayoutKey::printable(Scancode::MINUS, '\'', '?'),
    LayoutKey::printable(Scancode::EQUALS, 'ì', '^'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, 'è', 'é'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '+', '*'),
    LayoutKey::printable(Scancode::BACKSLASH, 'ù', '§'),
    LayoutKey::printable(Scancode::SEMICOLON, 'ò', 'ç'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'à', '°'),
    LayoutKey::printable(Scancode::GRAVE, '\\', '|'),
    LayoutKey::printable(Scancode::COMMA, ',', ';'),
    LayoutKey::printable(Scancode::PERIOD, '.', ':'),
    LayoutKey::printable(Scancode::SLASH, '-', '_'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
];

pub const LAYOUT_MAC_IT_ITALIAN: Layout = mk_layout(
    "mac/it-t-k0-italian",
    "Italiano — Italian",
    Platform::Mac,
    "it",
    ITALIAN_IT_KEYS,
);
pub const LAYOUT_WIN_IT_ITALIAN: Layout = mk_layout(
    "windows/it-t-k0-italian",
    "Italiano — Italian",
    Platform::Windows,
    "it",
    ITALIAN_IT_KEYS,
);
pub const LAYOUT_LINUX_IT_ITALIAN: Layout = mk_layout(
    "linux/it-t-k0-italian",
    "Italiano — Italian",
    Platform::Linux,
    "it",
    ITALIAN_IT_KEYS,
);

// ==========================================================================
// Português — Portuguese (Portugal)
// ==========================================================================

pub(crate) const PORTUGUESE_PT_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable(Scancode::NUM_7, '7', '/'),
    LayoutKey::printable(Scancode::NUM_8, '8', '('),
    LayoutKey::printable(Scancode::NUM_9, '9', ')'),
    LayoutKey::printable(Scancode::NUM_0, '0', '='),
    LayoutKey::printable(Scancode::MINUS, '\'', '?'),
    LayoutKey::printable(Scancode::EQUALS, '«', '»'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '+', '*'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '´', '`'),
    LayoutKey::printable(Scancode::BACKSLASH, '~', '^'),
    LayoutKey::printable(Scancode::SEMICOLON, 'ç', 'Ç'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'º', 'ª'),
    LayoutKey::printable(Scancode::GRAVE, '\\', '|'),
    LayoutKey::printable(Scancode::COMMA, ',', ';'),
    LayoutKey::printable(Scancode::PERIOD, '.', ':'),
    LayoutKey::printable(Scancode::SLASH, '-', '_'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
];

pub const LAYOUT_MAC_PT_PORTUGUESE: Layout = mk_layout(
    "mac/pt-t-k0-portuguese",
    "Português — Portuguese (Portugal)",
    Platform::Mac,
    "pt",
    PORTUGUESE_PT_KEYS,
);
pub const LAYOUT_WIN_PT_PORTUGUESE: Layout = mk_layout(
    "windows/pt-t-k0-portuguese",
    "Português — Portuguese (Portugal)",
    Platform::Windows,
    "pt",
    PORTUGUESE_PT_KEYS,
);
pub const LAYOUT_LINUX_PT_PORTUGUESE: Layout = mk_layout(
    "linux/pt-t-k0-portuguese",
    "Português — Portuguese (Portugal)",
    Platform::Linux,
    "pt",
    PORTUGUESE_PT_KEYS,
);

// ==========================================================================
// Português — Brazilian ABNT2
// ==========================================================================

pub(crate) const ABNT2_PT_BR_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '@'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '¨'),
    LayoutKey::printable(Scancode::NUM_7, '7', '&'),
    LayoutKey::printable(Scancode::NUM_8, '8', '*'),
    LayoutKey::printable(Scancode::NUM_9, '9', '('),
    LayoutKey::printable(Scancode::NUM_0, '0', ')'),
    LayoutKey::printable(Scancode::MINUS, '-', '_'),
    LayoutKey::printable(Scancode::EQUALS, '=', '+'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '´', '`'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '[', '{'),
    LayoutKey::printable(Scancode::BACKSLASH, ']', '}'),
    LayoutKey::printable(Scancode::SEMICOLON, 'ç', 'Ç'),
    LayoutKey::printable(Scancode::APOSTROPHE, '~', '^'),
    LayoutKey::printable(Scancode::GRAVE, '\'', '"'),
    LayoutKey::printable(Scancode::COMMA, ',', '<'),
    LayoutKey::printable(Scancode::PERIOD, '.', '>'),
    LayoutKey::printable(Scancode::SLASH, ';', ':'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '\\', '|'),
    // The ABNT2 extra key (INTERNATIONAL1) — '/' / '?'.
    LayoutKey::printable(Scancode::INTERNATIONAL1, '/', '?'),
];

pub const LAYOUT_MAC_PT_BR_ABNT2: Layout = mk_layout(
    "mac/pt-BR-t-k0-abnt2",
    "Português — Brazilian ABNT2",
    Platform::Mac,
    "pt",
    ABNT2_PT_BR_KEYS,
);
pub const LAYOUT_WIN_PT_BR_ABNT2: Layout = mk_layout(
    "windows/pt-BR-t-k0-abnt2",
    "Português — Brazilian ABNT2",
    Platform::Windows,
    "pt",
    ABNT2_PT_BR_KEYS,
);
pub const LAYOUT_LINUX_PT_BR_ABNT2: Layout = mk_layout(
    "linux/pt-BR-t-k0-abnt2",
    "Português — Brazilian ABNT2",
    Platform::Linux,
    "pt",
    ABNT2_PT_BR_KEYS,
);

// ==========================================================================
// Nordic layouts — Swedish / Finnish / Norwegian / Danish share a
// backbone; they diverge on the `[` / `'` / `;` rows (Ä/Ö vs Æ/Ø,
// etc.) and on the Shift-level of a few digits.
// ==========================================================================

pub(crate) const SWEDISH_SV_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '¤'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable(Scancode::NUM_7, '7', '/'),
    LayoutKey::printable(Scancode::NUM_8, '8', '('),
    LayoutKey::printable(Scancode::NUM_9, '9', ')'),
    LayoutKey::printable(Scancode::NUM_0, '0', '='),
    LayoutKey::printable(Scancode::MINUS, '+', '?'),
    LayoutKey::printable(Scancode::EQUALS, '´', '`'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, 'å', 'Å'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '¨', '^'),
    LayoutKey::printable(Scancode::BACKSLASH, '\'', '*'),
    LayoutKey::printable(Scancode::SEMICOLON, 'ö', 'Ö'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'ä', 'Ä'),
    LayoutKey::printable(Scancode::GRAVE, '§', '½'),
    LayoutKey::printable(Scancode::COMMA, ',', ';'),
    LayoutKey::printable(Scancode::PERIOD, '.', ':'),
    LayoutKey::printable(Scancode::SLASH, '-', '_'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
];

pub const LAYOUT_MAC_SV_SWEDISH: Layout = mk_layout(
    "mac/sv-t-k0-swedish",
    "Svenska — Swedish",
    Platform::Mac,
    "sv",
    SWEDISH_SV_KEYS,
);
pub const LAYOUT_WIN_SV_SWEDISH: Layout = mk_layout(
    "windows/sv-t-k0-swedish",
    "Svenska — Swedish",
    Platform::Windows,
    "sv",
    SWEDISH_SV_KEYS,
);
pub const LAYOUT_LINUX_SV_SWEDISH: Layout = mk_layout(
    "linux/sv-t-k0-swedish",
    "Svenska — Swedish",
    Platform::Linux,
    "sv",
    SWEDISH_SV_KEYS,
);

pub(crate) const FINNISH_FI_KEYS: &[LayoutKey] = SWEDISH_SV_KEYS;

pub const LAYOUT_MAC_FI_FINNISH: Layout = mk_layout(
    "mac/fi-t-k0-finnish",
    "Suomi — Finnish",
    Platform::Mac,
    "fi",
    FINNISH_FI_KEYS,
);
pub const LAYOUT_WIN_FI_FINNISH: Layout = mk_layout(
    "windows/fi-t-k0-finnish",
    "Suomi — Finnish",
    Platform::Windows,
    "fi",
    FINNISH_FI_KEYS,
);
pub const LAYOUT_LINUX_FI_FINNISH: Layout = mk_layout(
    "linux/fi-t-k0-finnish",
    "Suomi — Finnish",
    Platform::Linux,
    "fi",
    FINNISH_FI_KEYS,
);

pub(crate) const NORWEGIAN_NO_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '¤'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable(Scancode::NUM_7, '7', '/'),
    LayoutKey::printable(Scancode::NUM_8, '8', '('),
    LayoutKey::printable(Scancode::NUM_9, '9', ')'),
    LayoutKey::printable(Scancode::NUM_0, '0', '='),
    LayoutKey::printable(Scancode::MINUS, '+', '?'),
    LayoutKey::printable(Scancode::EQUALS, '\\', '`'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, 'å', 'Å'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '¨', '^'),
    LayoutKey::printable(Scancode::BACKSLASH, '\'', '*'),
    LayoutKey::printable(Scancode::SEMICOLON, 'ø', 'Ø'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'æ', 'Æ'),
    LayoutKey::printable(Scancode::GRAVE, '|', '§'),
    LayoutKey::printable(Scancode::COMMA, ',', ';'),
    LayoutKey::printable(Scancode::PERIOD, '.', ':'),
    LayoutKey::printable(Scancode::SLASH, '-', '_'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
];

pub const LAYOUT_MAC_NO_NORWEGIAN: Layout = mk_layout(
    "mac/nb-t-k0-norwegian",
    "Norsk — Norwegian",
    Platform::Mac,
    "nb",
    NORWEGIAN_NO_KEYS,
);
pub const LAYOUT_WIN_NO_NORWEGIAN: Layout = mk_layout(
    "windows/nb-t-k0-norwegian",
    "Norsk — Norwegian",
    Platform::Windows,
    "nb",
    NORWEGIAN_NO_KEYS,
);
pub const LAYOUT_LINUX_NO_NORWEGIAN: Layout = mk_layout(
    "linux/nb-t-k0-norwegian",
    "Norsk — Norwegian",
    Platform::Linux,
    "nb",
    NORWEGIAN_NO_KEYS,
);

pub(crate) const DANISH_DA_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '¤'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable(Scancode::NUM_7, '7', '/'),
    LayoutKey::printable(Scancode::NUM_8, '8', '('),
    LayoutKey::printable(Scancode::NUM_9, '9', ')'),
    LayoutKey::printable(Scancode::NUM_0, '0', '='),
    LayoutKey::printable(Scancode::MINUS, '+', '?'),
    LayoutKey::printable(Scancode::EQUALS, '´', '`'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, 'å', 'Å'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '¨', '^'),
    LayoutKey::printable(Scancode::BACKSLASH, '\'', '*'),
    LayoutKey::printable(Scancode::SEMICOLON, 'æ', 'Æ'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'ø', 'Ø'),
    LayoutKey::printable(Scancode::GRAVE, '½', '§'),
    LayoutKey::printable(Scancode::COMMA, ',', ';'),
    LayoutKey::printable(Scancode::PERIOD, '.', ':'),
    LayoutKey::printable(Scancode::SLASH, '-', '_'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
];

pub const LAYOUT_MAC_DA_DANISH: Layout = mk_layout(
    "mac/da-t-k0-danish",
    "Dansk — Danish",
    Platform::Mac,
    "da",
    DANISH_DA_KEYS,
);
pub const LAYOUT_WIN_DA_DANISH: Layout = mk_layout(
    "windows/da-t-k0-danish",
    "Dansk — Danish",
    Platform::Windows,
    "da",
    DANISH_DA_KEYS,
);
pub const LAYOUT_LINUX_DA_DANISH: Layout = mk_layout(
    "linux/da-t-k0-danish",
    "Dansk — Danish",
    Platform::Linux,
    "da",
    DANISH_DA_KEYS,
);

// ==========================================================================
// Türkçe — Turkish Q
// ==========================================================================

pub(crate) const TURKISH_Q_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    // Turkish distinguishes dotted İ/i and dotless I/ı.
    LayoutKey::printable(Scancode::I, 'ı', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '\''),
    LayoutKey::printable(Scancode::NUM_3, '3', '^'),
    LayoutKey::printable(Scancode::NUM_4, '4', '+'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable(Scancode::NUM_7, '7', '/'),
    LayoutKey::printable(Scancode::NUM_8, '8', '('),
    LayoutKey::printable(Scancode::NUM_9, '9', ')'),
    LayoutKey::printable(Scancode::NUM_0, '0', '='),
    LayoutKey::printable(Scancode::MINUS, '*', '?'),
    LayoutKey::printable(Scancode::EQUALS, '-', '_'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, 'ğ', 'Ğ'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, 'ü', 'Ü'),
    LayoutKey::printable(Scancode::BACKSLASH, ',', ';'),
    LayoutKey::printable(Scancode::SEMICOLON, 'ş', 'Ş'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'i', 'İ'),
    LayoutKey::printable(Scancode::GRAVE, '"', 'é'),
    LayoutKey::printable(Scancode::COMMA, 'ö', 'Ö'),
    LayoutKey::printable(Scancode::PERIOD, 'ç', 'Ç'),
    LayoutKey::printable(Scancode::SLASH, '.', ':'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, '<', '>'),
];

pub const LAYOUT_MAC_TR_TURKISH_Q: Layout = mk_layout(
    "mac/tr-t-k0-turkish-q",
    "Türkçe — Turkish Q",
    Platform::Mac,
    "tr",
    TURKISH_Q_KEYS,
);
pub const LAYOUT_WIN_TR_TURKISH_Q: Layout = mk_layout(
    "windows/tr-t-k0-turkish-q",
    "Türkçe — Turkish Q",
    Platform::Windows,
    "tr",
    TURKISH_Q_KEYS,
);
pub const LAYOUT_LINUX_TR_TURKISH_Q: Layout = mk_layout(
    "linux/tr-t-k0-turkish-q",
    "Türkçe — Turkish Q",
    Platform::Linux,
    "tr",
    TURKISH_Q_KEYS,
);

// ==========================================================================
// Polski — Polish Programmers (QWERTY base; AltGr for diacritics).
// ==========================================================================

pub(crate) const POLISH_PROG_KEYS: &[LayoutKey] = &[
    LayoutKey::printable4(Scancode::A, 'a', 'A', 'ą', 'Ą'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable4(Scancode::C, 'c', 'C', 'ć', 'Ć'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable4(Scancode::E, 'e', 'E', 'ę', 'Ę'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable4(Scancode::L, 'l', 'L', 'ł', 'Ł'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable4(Scancode::N, 'n', 'N', 'ń', 'Ń'),
    LayoutKey::printable4(Scancode::O, 'o', 'O', 'ó', 'Ó'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable4(Scancode::S, 's', 'S', 'ś', 'Ś'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable4(Scancode::X, 'x', 'X', 'ź', 'Ź'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable4(Scancode::Z, 'z', 'Z', 'ż', 'Ż'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '@'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '^'),
    LayoutKey::printable(Scancode::NUM_7, '7', '&'),
    LayoutKey::printable(Scancode::NUM_8, '8', '*'),
    LayoutKey::printable(Scancode::NUM_9, '9', '('),
    LayoutKey::printable(Scancode::NUM_0, '0', ')'),
    LayoutKey::printable(Scancode::MINUS, '-', '_'),
    LayoutKey::printable(Scancode::EQUALS, '=', '+'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '[', '{'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, ']', '}'),
    LayoutKey::printable(Scancode::BACKSLASH, '\\', '|'),
    LayoutKey::printable(Scancode::SEMICOLON, ';', ':'),
    LayoutKey::printable(Scancode::APOSTROPHE, '\'', '"'),
    LayoutKey::printable(Scancode::GRAVE, '`', '~'),
    LayoutKey::printable(Scancode::COMMA, ',', '<'),
    LayoutKey::printable(Scancode::PERIOD, '.', '>'),
    LayoutKey::printable(Scancode::SLASH, '/', '?'),
];

pub const LAYOUT_MAC_PL_PROG: Layout = mk_layout(
    "mac/pl-t-k0-polish-prog",
    "Polski — Programmers",
    Platform::Mac,
    "pl",
    POLISH_PROG_KEYS,
);
pub const LAYOUT_WIN_PL_PROG: Layout = mk_layout(
    "windows/pl-t-k0-polish-prog",
    "Polski — Programmers",
    Platform::Windows,
    "pl",
    POLISH_PROG_KEYS,
);
pub const LAYOUT_LINUX_PL_PROG: Layout = mk_layout(
    "linux/pl-t-k0-polish-prog",
    "Polski — Programmers",
    Platform::Linux,
    "pl",
    POLISH_PROG_KEYS,
);

// ==========================================================================
// Français — Canadian Multilingual (CSA)
// ==========================================================================

pub(crate) const FR_CA_CSA_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '/'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '?'),
    LayoutKey::printable(Scancode::NUM_7, '7', '&'),
    LayoutKey::printable(Scancode::NUM_8, '8', '*'),
    LayoutKey::printable(Scancode::NUM_9, '9', '('),
    LayoutKey::printable(Scancode::NUM_0, '0', ')'),
    LayoutKey::printable(Scancode::MINUS, '-', '_'),
    LayoutKey::printable(Scancode::EQUALS, '=', '+'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '^', '^'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, 'ç', 'Ç'),
    LayoutKey::printable(Scancode::BACKSLASH, '<', '>'),
    LayoutKey::printable(Scancode::SEMICOLON, ';', ':'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'è', 'È'),
    LayoutKey::printable(Scancode::GRAVE, '/', '\\'),
    LayoutKey::printable(Scancode::COMMA, ',', '\''),
    LayoutKey::printable(Scancode::PERIOD, '.', '.'),
    LayoutKey::printable(Scancode::SLASH, 'é', 'É'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, 'à', 'À'),
];

pub const LAYOUT_MAC_FR_CA: Layout = mk_layout(
    "mac/fr-CA-t-k0-csa",
    "Français — Canadian Multilingual",
    Platform::Mac,
    "fr",
    FR_CA_CSA_KEYS,
);
pub const LAYOUT_WIN_FR_CA: Layout = mk_layout(
    "windows/fr-CA-t-k0-csa",
    "Français — Canadian Multilingual",
    Platform::Windows,
    "fr",
    FR_CA_CSA_KEYS,
);
pub const LAYOUT_LINUX_FR_CA: Layout = mk_layout(
    "linux/fr-CA-t-k0-csa",
    "Français — Canadian Multilingual",
    Platform::Linux,
    "fr",
    FR_CA_CSA_KEYS,
);

// ==========================================================================
// Français — BÉPO (Dvorak-inspired French)
// ==========================================================================

pub(crate) const BEPO_FR_KEYS: &[LayoutKey] = &[
    // Top letter row: B É P O È ^ V D L J Z W
    LayoutKey::printable(Scancode::Q, 'b', 'B'),
    LayoutKey::printable(Scancode::W, 'é', 'É'),
    LayoutKey::printable(Scancode::E, 'p', 'P'),
    LayoutKey::printable(Scancode::R, 'o', 'O'),
    LayoutKey::printable(Scancode::T, 'è', 'È'),
    LayoutKey::printable(Scancode::Y, '^', '!'),
    LayoutKey::printable(Scancode::U, 'v', 'V'),
    LayoutKey::printable(Scancode::I, 'd', 'D'),
    LayoutKey::printable(Scancode::O, 'l', 'L'),
    LayoutKey::printable(Scancode::P, 'j', 'J'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, 'z', 'Z'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, 'w', 'W'),
    // Home row: A U I E , C T S R N M Ç
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::S, 'u', 'U'),
    LayoutKey::printable(Scancode::D, 'i', 'I'),
    LayoutKey::printable(Scancode::F, 'e', 'E'),
    LayoutKey::printable(Scancode::G, ',', ';'),
    LayoutKey::printable(Scancode::H, 'c', 'C'),
    LayoutKey::printable(Scancode::J, 't', 'T'),
    LayoutKey::printable(Scancode::K, 's', 'S'),
    LayoutKey::printable(Scancode::L, 'r', 'R'),
    LayoutKey::printable(Scancode::SEMICOLON, 'n', 'N'),
    LayoutKey::printable(Scancode::APOSTROPHE, 'm', 'M'),
    // Bottom row: Ê À Y X . K ' Q G H F
    LayoutKey::printable(Scancode::Z, 'ê', 'Ê'),
    LayoutKey::printable(Scancode::X, 'à', 'À'),
    LayoutKey::printable(Scancode::C, 'y', 'Y'),
    LayoutKey::printable(Scancode::V, 'x', 'X'),
    LayoutKey::printable(Scancode::B, '.', ':'),
    LayoutKey::printable(Scancode::N, 'k', 'K'),
    LayoutKey::printable(Scancode::M, '\'', '?'),
    LayoutKey::printable(Scancode::COMMA, 'q', 'Q'),
    LayoutKey::printable(Scancode::PERIOD, 'g', 'G'),
    LayoutKey::printable(Scancode::SLASH, 'h', 'H'),
    LayoutKey::printable(Scancode::NON_US_BACKSLASH, 'ç', 'Ç'),
    LayoutKey::printable(Scancode::BACKSLASH, 'f', 'F'),
    // Digit row — BÉPO unshifted gives typographic punctuation.
    LayoutKey::printable(Scancode::NUM_1, '"', '1'),
    LayoutKey::printable(Scancode::NUM_2, '«', '2'),
    LayoutKey::printable(Scancode::NUM_3, '»', '3'),
    LayoutKey::printable(Scancode::NUM_4, '(', '4'),
    LayoutKey::printable(Scancode::NUM_5, ')', '5'),
    LayoutKey::printable(Scancode::NUM_6, '@', '6'),
    LayoutKey::printable(Scancode::NUM_7, '+', '7'),
    LayoutKey::printable(Scancode::NUM_8, '-', '8'),
    LayoutKey::printable(Scancode::NUM_9, '/', '9'),
    LayoutKey::printable(Scancode::NUM_0, '*', '0'),
    LayoutKey::printable(Scancode::MINUS, '=', '°'),
    LayoutKey::printable(Scancode::EQUALS, '%', '`'),
    LayoutKey::printable(Scancode::GRAVE, '$', '#'),
];

pub const LAYOUT_MAC_FR_BEPO: Layout = mk_layout(
    "mac/fr-t-k0-bepo",
    "Français — BÉPO",
    Platform::Mac,
    "fr",
    BEPO_FR_KEYS,
);
pub const LAYOUT_WIN_FR_BEPO: Layout = mk_layout(
    "windows/fr-t-k0-bepo",
    "Français — BÉPO",
    Platform::Windows,
    "fr",
    BEPO_FR_KEYS,
);
pub const LAYOUT_LINUX_FR_BEPO: Layout = mk_layout(
    "linux/fr-t-k0-bepo",
    "Français — BÉPO",
    Platform::Linux,
    "fr",
    BEPO_FR_KEYS,
);

// ==========================================================================
// 日本語 — Japanese JIS (ASCII-compatible with yen on BACKSLASH; kana
// modes are IME-driven, out of scope for this layer)
// ==========================================================================

pub(crate) const JIS_JA_KEYS: &[LayoutKey] = &[
    LayoutKey::printable(Scancode::A, 'a', 'A'),
    LayoutKey::printable(Scancode::B, 'b', 'B'),
    LayoutKey::printable(Scancode::C, 'c', 'C'),
    LayoutKey::printable(Scancode::D, 'd', 'D'),
    LayoutKey::printable(Scancode::E, 'e', 'E'),
    LayoutKey::printable(Scancode::F, 'f', 'F'),
    LayoutKey::printable(Scancode::G, 'g', 'G'),
    LayoutKey::printable(Scancode::H, 'h', 'H'),
    LayoutKey::printable(Scancode::I, 'i', 'I'),
    LayoutKey::printable(Scancode::J, 'j', 'J'),
    LayoutKey::printable(Scancode::K, 'k', 'K'),
    LayoutKey::printable(Scancode::L, 'l', 'L'),
    LayoutKey::printable(Scancode::M, 'm', 'M'),
    LayoutKey::printable(Scancode::N, 'n', 'N'),
    LayoutKey::printable(Scancode::O, 'o', 'O'),
    LayoutKey::printable(Scancode::P, 'p', 'P'),
    LayoutKey::printable(Scancode::Q, 'q', 'Q'),
    LayoutKey::printable(Scancode::R, 'r', 'R'),
    LayoutKey::printable(Scancode::S, 's', 'S'),
    LayoutKey::printable(Scancode::T, 't', 'T'),
    LayoutKey::printable(Scancode::U, 'u', 'U'),
    LayoutKey::printable(Scancode::V, 'v', 'V'),
    LayoutKey::printable(Scancode::W, 'w', 'W'),
    LayoutKey::printable(Scancode::X, 'x', 'X'),
    LayoutKey::printable(Scancode::Y, 'y', 'Y'),
    LayoutKey::printable(Scancode::Z, 'z', 'Z'),
    LayoutKey::printable(Scancode::NUM_1, '1', '!'),
    LayoutKey::printable(Scancode::NUM_2, '2', '"'),
    LayoutKey::printable(Scancode::NUM_3, '3', '#'),
    LayoutKey::printable(Scancode::NUM_4, '4', '$'),
    LayoutKey::printable(Scancode::NUM_5, '5', '%'),
    LayoutKey::printable(Scancode::NUM_6, '6', '&'),
    LayoutKey::printable(Scancode::NUM_7, '7', '\''),
    LayoutKey::printable(Scancode::NUM_8, '8', '('),
    LayoutKey::printable(Scancode::NUM_9, '9', ')'),
    LayoutKey::printable(Scancode::NUM_0, '0', '\0'),
    LayoutKey::printable(Scancode::MINUS, '-', '='),
    LayoutKey::printable(Scancode::EQUALS, '^', '~'),
    LayoutKey::printable(Scancode::LEFT_BRACKET, '@', '`'),
    LayoutKey::printable(Scancode::RIGHT_BRACKET, '[', '{'),
    // The yen / backslash key on JIS layouts.
    LayoutKey::printable(Scancode::BACKSLASH, '¥', '|'),
    LayoutKey::printable(Scancode::SEMICOLON, ';', '+'),
    LayoutKey::printable(Scancode::APOSTROPHE, ':', '*'),
    LayoutKey::printable(Scancode::GRAVE, ']', '}'),
    LayoutKey::printable(Scancode::COMMA, ',', '<'),
    LayoutKey::printable(Scancode::PERIOD, '.', '>'),
    LayoutKey::printable(Scancode::SLASH, '/', '?'),
    // The extra JIS "ろ" / underscore key.
    LayoutKey::printable(Scancode::INTERNATIONAL1, '\\', '_'),
];

pub const LAYOUT_MAC_JA_JIS: Layout = mk_layout(
    "mac/ja-t-k0-jis",
    "日本語 — JIS",
    Platform::Mac,
    "ja",
    JIS_JA_KEYS,
);
pub const LAYOUT_WIN_JA_JIS: Layout = mk_layout(
    "windows/ja-t-k0-jis",
    "日本語 — JIS",
    Platform::Windows,
    "ja",
    JIS_JA_KEYS,
);
pub const LAYOUT_LINUX_JA_JIS: Layout = mk_layout(
    "linux/ja-t-k0-jis",
    "日本語 — JIS",
    Platform::Linux,
    "ja",
    JIS_JA_KEYS,
);

// ==========================================================================
// CLDR-generated layouts (build.rs emits $OUT_DIR/cldr_layouts.rs from
// data/cldr-43/keyboards/).
// ==========================================================================

include!(concat!(env!("OUT_DIR"), "/cldr_layouts.rs"));

// ==========================================================================
// Registry
// ==========================================================================

/// Hand-curated layouts shipped directly in source.
pub const HAND_CODED_LAYOUTS: &[&Layout] = &[
    // English
    &LAYOUT_MAC_EN_US_QWERTY,
    &LAYOUT_WIN_EN_US_QWERTY,
    &LAYOUT_LINUX_EN_US_QWERTY,
    &LAYOUT_MAC_EN_GB_QWERTY,
    &LAYOUT_WIN_EN_GB_QWERTY,
    &LAYOUT_LINUX_EN_GB_QWERTY,
    &LAYOUT_MAC_EN_US_INTL,
    &LAYOUT_WIN_EN_US_INTL,
    &LAYOUT_LINUX_EN_US_INTL,
    &LAYOUT_MAC_EN_US_DVORAK,
    &LAYOUT_WIN_EN_US_DVORAK,
    &LAYOUT_LINUX_EN_US_DVORAK,
    &LAYOUT_MAC_EN_US_COLEMAK,
    &LAYOUT_WIN_EN_US_COLEMAK,
    &LAYOUT_LINUX_EN_US_COLEMAK,
    // French
    &LAYOUT_MAC_FR_AZERTY,
    &LAYOUT_WIN_FR_AZERTY,
    &LAYOUT_LINUX_FR_AZERTY,
    &LAYOUT_MAC_FR_CA,
    &LAYOUT_WIN_FR_CA,
    &LAYOUT_LINUX_FR_CA,
    &LAYOUT_MAC_FR_BEPO,
    &LAYOUT_WIN_FR_BEPO,
    &LAYOUT_LINUX_FR_BEPO,
    // German
    &LAYOUT_MAC_DE_QWERTZ,
    &LAYOUT_WIN_DE_QWERTZ,
    &LAYOUT_LINUX_DE_QWERTZ,
    // Spanish
    &LAYOUT_MAC_ES_SPANISH,
    &LAYOUT_WIN_ES_SPANISH,
    &LAYOUT_LINUX_ES_SPANISH,
    &LAYOUT_MAC_ES_LATAM,
    &LAYOUT_WIN_ES_LATAM,
    &LAYOUT_LINUX_ES_LATAM,
    // Italian
    &LAYOUT_MAC_IT_ITALIAN,
    &LAYOUT_WIN_IT_ITALIAN,
    &LAYOUT_LINUX_IT_ITALIAN,
    // Portuguese
    &LAYOUT_MAC_PT_PORTUGUESE,
    &LAYOUT_WIN_PT_PORTUGUESE,
    &LAYOUT_LINUX_PT_PORTUGUESE,
    &LAYOUT_MAC_PT_BR_ABNT2,
    &LAYOUT_WIN_PT_BR_ABNT2,
    &LAYOUT_LINUX_PT_BR_ABNT2,
    // Nordic
    &LAYOUT_MAC_SV_SWEDISH,
    &LAYOUT_WIN_SV_SWEDISH,
    &LAYOUT_LINUX_SV_SWEDISH,
    &LAYOUT_MAC_FI_FINNISH,
    &LAYOUT_WIN_FI_FINNISH,
    &LAYOUT_LINUX_FI_FINNISH,
    &LAYOUT_MAC_NO_NORWEGIAN,
    &LAYOUT_WIN_NO_NORWEGIAN,
    &LAYOUT_LINUX_NO_NORWEGIAN,
    &LAYOUT_MAC_DA_DANISH,
    &LAYOUT_WIN_DA_DANISH,
    &LAYOUT_LINUX_DA_DANISH,
    // Slavic / Baltic
    &LAYOUT_MAC_PL_PROG,
    &LAYOUT_WIN_PL_PROG,
    &LAYOUT_LINUX_PL_PROG,
    &LAYOUT_MAC_RU_JCUKEN,
    &LAYOUT_WIN_RU_JCUKEN,
    &LAYOUT_LINUX_RU_JCUKEN,
    // Turkish
    &LAYOUT_MAC_TR_TURKISH_Q,
    &LAYOUT_WIN_TR_TURKISH_Q,
    &LAYOUT_LINUX_TR_TURKISH_Q,
    // East Asian
    &LAYOUT_MAC_JA_JIS,
    &LAYOUT_WIN_JA_JIS,
    &LAYOUT_LINUX_JA_JIS,
];

/// Look up a layout by its id (e.g. `"linux/fr-t-k0-azerty"`). Checks
/// hand-curated layouts first, then falls through to the CLDR-generated
/// set — so a hand-coded layout always wins over an auto-generated one
/// sharing the same id.
pub fn get_layout(id: &str) -> Option<&'static Layout> {
    HAND_CODED_LAYOUTS
        .iter()
        .find(|l| l.id == id)
        .or_else(|| CLDR_LAYOUTS.iter().find(|l| l.id == id))
        .copied()
}

/// Every layout shipped in this build — the concatenation of
/// [`HAND_CODED_LAYOUTS`] and [`CLDR_LAYOUTS`], allocated once on first
/// call and returned as a borrowed slice.
pub fn all_layouts() -> &'static [&'static Layout] {
    use std::sync::OnceLock;
    static COMBINED: OnceLock<Vec<&'static Layout>> = OnceLock::new();
    COMBINED
        .get_or_init(|| {
            let mut v: Vec<&'static Layout> = HAND_CODED_LAYOUTS.to_vec();
            v.extend(CLDR_LAYOUTS.iter().copied());
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
