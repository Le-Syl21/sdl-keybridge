//! Deutsch (`de`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        ("key_escape", Textual) => "Esc",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "Eingabe",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "Tab",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "Leertaste",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "Rücktaste",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "Einfg",
        ("key_delete", Textual) => "Entf",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", Textual) => "Pos1",
        ("key_home", Symbolic) => "↖",
        ("key_end", Textual) => "Ende",
        ("key_end", Symbolic) => "↘",
        ("key_page_up", Textual) => "Bild ↑",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "Bild ↓",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "Hoch",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "Runter",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "Links",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "Rechts",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "Feststell",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "Num",
        ("key_scroll_lock", _) => "Rollen",
        ("key_print_screen", _) => "Druck",
        ("key_pause", _) => "Pause",
        ("key_menu", _) => "Menü",
        ("key_application", _) => "Anwendung",

        ("key_shift_left", Textual) => "Umschalt links",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "Umschalt rechts",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "Strg links",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "Strg rechts",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "Alt links",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "Alt rechts",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "Befehl links",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "Befehl rechts",
        ("key_gui_right", Symbolic) => "◇",

        ("mod_ctrl_mac", Textual) => "Control",
        ("mod_ctrl_mac", Symbolic) => "⌃",
        ("mod_ctrl_win", _)
        | ("mod_ctrl_linux", _)
        | ("mod_ctrl_chromeos", _)
        | ("mod_ctrl_android", _)
        | ("mod_ctrl", _) => "Strg",

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
        | ("mod_shift", Textual) => "Umschalt",

        ("mod_alt_mac", Textual) => "Option",
        ("mod_alt_mac", Symbolic) => "⌥",
        ("mod_alt_win", _)
        | ("mod_alt_linux", _)
        | ("mod_alt_chromeos", _)
        | ("mod_alt_android", _)
        | ("mod_alt", _) => "Alt",

        ("mod_gui_mac", Textual) => "Befehl",
        ("mod_gui_mac", Symbolic) => "⌘",
        ("mod_gui_win", Textual) => "Windows",
        ("mod_gui_win", Symbolic) => "⊞",
        ("mod_gui_linux", Textual) => "Super",
        ("mod_gui_linux", Symbolic) => "◇",
        ("mod_gui_chromeos", Textual) => "Suche",
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
