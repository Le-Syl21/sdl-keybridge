//! Italiano (`it`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        ("key_escape", Textual) => "Esc",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "Invio",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "Tab",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "Spazio",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "Backspace",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "Ins",
        ("key_delete", Textual) => "Canc",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", _) => "Inizio",
        ("key_end", _) => "Fine",
        ("key_page_up", Textual) => "Pag ↑",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "Pag ↓",
        ("key_page_down", Symbolic) => "⇟",
        ("key_arrow_up", Textual) => "Su",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "Giù",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "Sinistra",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "Destra",
        ("key_arrow_right", Symbolic) => "→",
        ("key_caps_lock", Textual) => "Bloc Maiusc",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "Bloc Num",
        ("key_scroll_lock", _) => "Bloc Scorr",
        ("key_print_screen", _) => "Stamp",
        ("key_pause", _) => "Pausa",
        ("key_menu", _) => "Menu",
        ("key_altgr", _) => "AltGr",
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
        | ("mod_shift", Textual) => "Maiusc",
        ("mod_alt_mac", Textual) => "Opzione",
        ("mod_alt_mac", Symbolic) => "⌥",
        ("mod_alt_win", _)
        | ("mod_alt_linux", _)
        | ("mod_alt_chromeos", _)
        | ("mod_alt_android", _)
        | ("mod_alt", _) => "Alt",
        ("mod_gui_mac", Textual) => "Comando",
        ("mod_gui_mac", Symbolic) => "⌘",
        ("mod_gui_win", Textual) => "Windows",
        ("mod_gui_win", Symbolic) => "⊞",
        ("mod_gui_linux", Textual) => "Super",
        ("mod_gui_linux", Symbolic) => "◇",
        ("mod_gui_chromeos", Textual) => "Cerca",
        ("mod_gui_chromeos", Symbolic) => "◯",
        ("mod_gui_android", _) | ("mod_gui", _) => "Meta",
        ("mod_altgr_mac", Textual) => "Opzione",
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
