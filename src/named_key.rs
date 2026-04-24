//! Taxonomy of named (non-printable) keys.
//!
//! Each variant has a stable string id (`"key_*"`) used as the lookup key
//! by [`crate::KeyLocalizer`] implementations. The id is the contract:
//! locale modules must produce a translation for every id that appears here.

/// Non-printable keys that can carry a localized label.
///
/// Keys whose label *is* a character (letters, digits, punctuation) are
/// not represented here — their label is the character itself, possibly
/// localized at a higher level by a glyph mapping per layout.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub enum NamedKey {
    Escape,
    Return,
    Tab,
    Space,
    Backspace,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    CapsLock,
    NumLock,
    ScrollLock,
    PrintScreen,
    Pause,
    Menu,
    Application,

    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    AltGr,
    GuiLeft,
    GuiRight,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    KeypadEnter,
    KeypadDivide,
    KeypadMultiply,
    KeypadMinus,
    KeypadPlus,
    KeypadPeriod,
    KeypadEquals,
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
}

impl NamedKey {
    /// Stable textual id — used by [`crate::KeyLocalizer::translate`].
    ///
    /// The id is considered part of the public contract: adding a new
    /// variant is fine, but never rename an existing id.
    pub const fn key_id(self) -> &'static str {
        use NamedKey::*;
        match self {
            Escape => "key_escape",
            Return => "key_return",
            Tab => "key_tab",
            Space => "key_space",
            Backspace => "key_backspace",
            Insert => "key_insert",
            Delete => "key_delete",
            Home => "key_home",
            End => "key_end",
            PageUp => "key_page_up",
            PageDown => "key_page_down",
            ArrowUp => "key_arrow_up",
            ArrowDown => "key_arrow_down",
            ArrowLeft => "key_arrow_left",
            ArrowRight => "key_arrow_right",

            CapsLock => "key_caps_lock",
            NumLock => "key_num_lock",
            ScrollLock => "key_scroll_lock",
            PrintScreen => "key_print_screen",
            Pause => "key_pause",
            Menu => "key_menu",
            Application => "key_application",

            ShiftLeft => "key_shift_left",
            ShiftRight => "key_shift_right",
            ControlLeft => "key_control_left",
            ControlRight => "key_control_right",
            AltLeft => "key_alt_left",
            AltRight => "key_alt_right",
            AltGr => "key_altgr",
            GuiLeft => "key_gui_left",
            GuiRight => "key_gui_right",

            F1 => "key_f1",
            F2 => "key_f2",
            F3 => "key_f3",
            F4 => "key_f4",
            F5 => "key_f5",
            F6 => "key_f6",
            F7 => "key_f7",
            F8 => "key_f8",
            F9 => "key_f9",
            F10 => "key_f10",
            F11 => "key_f11",
            F12 => "key_f12",
            F13 => "key_f13",
            F14 => "key_f14",
            F15 => "key_f15",
            F16 => "key_f16",
            F17 => "key_f17",
            F18 => "key_f18",
            F19 => "key_f19",
            F20 => "key_f20",
            F21 => "key_f21",
            F22 => "key_f22",
            F23 => "key_f23",
            F24 => "key_f24",

            KeypadEnter => "key_kp_enter",
            KeypadDivide => "key_kp_divide",
            KeypadMultiply => "key_kp_multiply",
            KeypadMinus => "key_kp_minus",
            KeypadPlus => "key_kp_plus",
            KeypadPeriod => "key_kp_period",
            KeypadEquals => "key_kp_equals",
            Keypad0 => "key_kp_0",
            Keypad1 => "key_kp_1",
            Keypad2 => "key_kp_2",
            Keypad3 => "key_kp_3",
            Keypad4 => "key_kp_4",
            Keypad5 => "key_kp_5",
            Keypad6 => "key_kp_6",
            Keypad7 => "key_kp_7",
            Keypad8 => "key_kp_8",
            Keypad9 => "key_kp_9",
        }
    }

    /// Iterator over every [`NamedKey`] variant. Useful in locale tests.
    pub fn all() -> &'static [NamedKey] {
        use NamedKey::*;
        &[
            Escape,
            Return,
            Tab,
            Space,
            Backspace,
            Insert,
            Delete,
            Home,
            End,
            PageUp,
            PageDown,
            ArrowUp,
            ArrowDown,
            ArrowLeft,
            ArrowRight,
            CapsLock,
            NumLock,
            ScrollLock,
            PrintScreen,
            Pause,
            Menu,
            Application,
            ShiftLeft,
            ShiftRight,
            ControlLeft,
            ControlRight,
            AltLeft,
            AltRight,
            AltGr,
            GuiLeft,
            GuiRight,
            F1,
            F2,
            F3,
            F4,
            F5,
            F6,
            F7,
            F8,
            F9,
            F10,
            F11,
            F12,
            F13,
            F14,
            F15,
            F16,
            F17,
            F18,
            F19,
            F20,
            F21,
            F22,
            F23,
            F24,
            KeypadEnter,
            KeypadDivide,
            KeypadMultiply,
            KeypadMinus,
            KeypadPlus,
            KeypadPeriod,
            KeypadEquals,
            Keypad0,
            Keypad1,
            Keypad2,
            Keypad3,
            Keypad4,
            Keypad5,
            Keypad6,
            Keypad7,
            Keypad8,
            Keypad9,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_ids_are_unique() {
        let ids: Vec<_> = NamedKey::all().iter().map(|k| k.key_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(ids.len(), sorted.len(), "duplicate key_id detected");
    }

    #[test]
    fn key_ids_have_prefix() {
        for k in NamedKey::all() {
            assert!(k.key_id().starts_with("key_"), "{:?}", k);
        }
    }
}
