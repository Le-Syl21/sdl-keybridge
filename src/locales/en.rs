//! English (`en`) — the reference locale.
//!
//! Every other locale module mirrors the structure of this file: a
//! single function `translate(key_id, style) -> Option<Cow<'static, str>>`
//! covering all `NamedKey::key_id()`s and every `mod_*` id emitted by
//! [`crate::modifier_label`].

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        // Editing / navigation keys.
        ("key_escape", Textual) => "Esc",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "Enter",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "Tab",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "Space",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "Backspace",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "Insert",
        ("key_delete", Textual) => "Delete",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", Textual) => "Home",
        ("key_home", Symbolic) => "↖",
        ("key_end", Textual) => "End",
        ("key_end", Symbolic) => "↘",
        ("key_page_up", Textual) => "Page Up",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "Page Down",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "Up",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "Down",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "Left",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "Right",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "Caps Lock",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "Num Lock",
        ("key_scroll_lock", _) => "Scroll Lock",
        ("key_print_screen", _) => "Print Screen",
        ("key_pause", _) => "Pause",
        ("key_menu", _) => "Menu",
        ("key_application", _) => "Application",

        // Modifier keys (as keys, not as held-modifier labels).
        ("key_shift_left", Textual) => "Left Shift",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "Right Shift",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "Left Ctrl",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "Right Ctrl",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "Left Alt",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "Right Alt",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "Left GUI",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "Right GUI",
        ("key_gui_right", Symbolic) => "◇",

        // Function keys.
        ("key_f1", _) => "F1",
        ("key_f2", _) => "F2",
        ("key_f3", _) => "F3",
        ("key_f4", _) => "F4",
        ("key_f5", _) => "F5",
        ("key_f6", _) => "F6",
        ("key_f7", _) => "F7",
        ("key_f8", _) => "F8",
        ("key_f9", _) => "F9",
        ("key_f10", _) => "F10",
        ("key_f11", _) => "F11",
        ("key_f12", _) => "F12",
        ("key_f13", _) => "F13",
        ("key_f14", _) => "F14",
        ("key_f15", _) => "F15",
        ("key_f16", _) => "F16",
        ("key_f17", _) => "F17",
        ("key_f18", _) => "F18",
        ("key_f19", _) => "F19",
        ("key_f20", _) => "F20",
        ("key_f21", _) => "F21",
        ("key_f22", _) => "F22",
        ("key_f23", _) => "F23",
        ("key_f24", _) => "F24",

        // Keypad keys.
        ("key_kp_enter", Textual) => "Keypad Enter",
        ("key_kp_enter", Symbolic) => "⏎",
        ("key_kp_divide", Textual) => "Keypad /",
        ("key_kp_divide", Symbolic) => "÷",
        ("key_kp_multiply", Textual) => "Keypad *",
        ("key_kp_multiply", Symbolic) => "×",
        ("key_kp_minus", Textual) => "Keypad -",
        ("key_kp_minus", Symbolic) => "−",
        ("key_kp_plus", Textual) => "Keypad +",
        ("key_kp_plus", Symbolic) => "+",
        ("key_kp_period", _) => "Keypad .",
        ("key_kp_equals", _) => "Keypad =",
        ("key_kp_0", _) => "Keypad 0",
        ("key_kp_1", _) => "Keypad 1",
        ("key_kp_2", _) => "Keypad 2",
        ("key_kp_3", _) => "Keypad 3",
        ("key_kp_4", _) => "Keypad 4",
        ("key_kp_5", _) => "Keypad 5",
        ("key_kp_6", _) => "Keypad 6",
        ("key_kp_7", _) => "Keypad 7",
        ("key_kp_8", _) => "Keypad 8",
        ("key_kp_9", _) => "Keypad 9",

        // Modifier labels — platform-aware.
        ("mod_ctrl_mac", Textual) => "Control",
        ("mod_ctrl_mac", Symbolic) => "⌃",
        ("mod_ctrl_win", _)
        | ("mod_ctrl_linux", _)
        | ("mod_ctrl_chromeos", _)
        | ("mod_ctrl_android", _)
        | ("mod_ctrl", _) => "Ctrl",

        ("mod_shift_mac", Symbolic)
        | ("mod_shift_win", Symbolic)
        | ("mod_shift_linux", Symbolic)
        | ("mod_shift_chromeos", Symbolic)
        | ("mod_shift_android", Symbolic)
        | ("mod_shift", Symbolic) => "⇧",
        ("mod_shift_mac", Textual)
        | ("mod_shift_win", Textual)
        | ("mod_shift_linux", Textual)
        | ("mod_shift_chromeos", Textual)
        | ("mod_shift_android", Textual)
        | ("mod_shift", Textual) => "Shift",

        ("mod_alt_mac", Textual) => "Option",
        ("mod_alt_mac", Symbolic) => "⌥",
        ("mod_alt_win", _)
        | ("mod_alt_linux", _)
        | ("mod_alt_chromeos", _)
        | ("mod_alt_android", _)
        | ("mod_alt", _) => "Alt",

        ("mod_gui_mac", Textual) => "Command",
        ("mod_gui_mac", Symbolic) => "⌘",
        ("mod_gui_win", Textual) => "Windows",
        ("mod_gui_win", Symbolic) => "⊞",
        ("mod_gui_linux", Textual) => "Super",
        ("mod_gui_linux", Symbolic) => "◇",
        ("mod_gui_chromeos", Textual) => "Search",
        ("mod_gui_chromeos", Symbolic) => "◯",
        ("mod_gui_android", _) => "Meta",
        ("mod_gui", _) => "Meta",

        ("mod_altgr_mac", Textual) => "Option",
        ("mod_altgr_mac", Symbolic) => "⌥",
        ("mod_altgr_win", _)
        | ("mod_altgr_linux", _)
        | ("mod_altgr_chromeos", _)
        | ("mod_altgr_android", _)
        | ("mod_altgr", _) => "AltGr",

        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
