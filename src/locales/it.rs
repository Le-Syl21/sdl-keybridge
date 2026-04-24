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
        ("key_application", _) => "Applicazione",

        ("key_shift_left", Textual) => "Maiusc sx",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "Maiusc dx",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "Ctrl sx",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "Ctrl dx",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "Alt sx",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "Alt dx",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "Cmd sx",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "Cmd dx",
        ("key_gui_right", Symbolic) => "◇",

        ("key_kp_enter", Textual) => "Invio (Tn)",
        ("key_kp_enter", Symbolic) => "⏎",
        ("key_kp_divide", Textual) => "Tn /",
        ("key_kp_divide", Symbolic) => "÷",
        ("key_kp_multiply", Textual) => "Tn *",
        ("key_kp_multiply", Symbolic) => "×",
        ("key_kp_minus", Textual) => "Tn -",
        ("key_kp_minus", Symbolic) => "−",
        ("key_kp_plus", Textual) => "Tn +",
        ("key_kp_plus", Symbolic) => "+",
        ("key_kp_period", _) => "Tn .",
        ("key_kp_equals", _) => "Tn =",
        ("key_kp_0", _) => "Tn 0",
        ("key_kp_1", _) => "Tn 1",
        ("key_kp_2", _) => "Tn 2",
        ("key_kp_3", _) => "Tn 3",
        ("key_kp_4", _) => "Tn 4",
        ("key_kp_5", _) => "Tn 5",
        ("key_kp_6", _) => "Tn 6",
        ("key_kp_7", _) => "Tn 7",
        ("key_kp_8", _) => "Tn 8",
        ("key_kp_9", _) => "Tn 9",

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
