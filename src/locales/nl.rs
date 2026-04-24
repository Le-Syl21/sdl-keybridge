//! Nederlands (`nl`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        ("key_escape", Textual) => "Esc",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "Enter",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", _) => "Tab",
        ("key_space", Textual) => "Spatie",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "Backspace",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "Invoegen",
        ("key_delete", Textual) => "Verwijderen",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", _) => "Home",
        ("key_end", _) => "End",
        ("key_page_up", Textual) => "Page Up",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "Page Down",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "Omhoog",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "Omlaag",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "Links",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "Rechts",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "Caps Lock",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "Num Lock",
        ("key_scroll_lock", _) => "Scroll Lock",
        ("key_print_screen", _) => "Print Screen",
        ("key_pause", _) => "Pauze",
        ("key_menu", _) => "Menu",
        ("key_application", _) => "Applicatie",

        ("key_shift_left", Textual) => "Shift links",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "Shift rechts",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "Ctrl links",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "Ctrl rechts",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "Alt links",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "Alt rechts",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "Cmd links",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "Cmd rechts",
        ("key_gui_right", Symbolic) => "◇",

        ("key_kp_enter", Textual) => "Enter (Num)",
        ("key_kp_enter", Symbolic) => "⏎",
        ("key_kp_divide", Textual) => "Num /",
        ("key_kp_divide", Symbolic) => "÷",
        ("key_kp_multiply", Textual) => "Num *",
        ("key_kp_multiply", Symbolic) => "×",
        ("key_kp_minus", Textual) => "Num -",
        ("key_kp_minus", Symbolic) => "−",
        ("key_kp_plus", Textual) => "Num +",
        ("key_kp_plus", Symbolic) => "+",
        ("key_kp_period", _) => "Num .",
        ("key_kp_equals", _) => "Num =",
        ("key_kp_0", _) => "Num 0",
        ("key_kp_1", _) => "Num 1",
        ("key_kp_2", _) => "Num 2",
        ("key_kp_3", _) => "Num 3",
        ("key_kp_4", _) => "Num 4",
        ("key_kp_5", _) => "Num 5",
        ("key_kp_6", _) => "Num 6",
        ("key_kp_7", _) => "Num 7",
        ("key_kp_8", _) => "Num 8",
        ("key_kp_9", _) => "Num 9",

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

        ("mod_gui_mac", Textual) => "Commando",
        ("mod_gui_mac", Symbolic) => "⌘",
        ("mod_gui_win", Textual) => "Windows",
        ("mod_gui_win", Symbolic) => "⊞",
        ("mod_gui_linux", Textual) => "Super",
        ("mod_gui_linux", Symbolic) => "◇",
        ("mod_gui_chromeos", Textual) => "Zoeken",
        ("mod_gui_chromeos", Symbolic) => "◯",
        ("mod_gui_android", _) | ("mod_gui", _) => "Meta",

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
