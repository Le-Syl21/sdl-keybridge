//! Русский (`ru`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        ("key_escape", Textual) => "Esc",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "Ввод",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "Tab",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "Пробел",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "Backspace",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "Вставка",
        ("key_delete", Textual) => "Удал",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", _) => "Home",
        ("key_end", _) => "End",
        ("key_page_up", Textual) => "Страница вверх",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "Страница вниз",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "Вверх",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "Вниз",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "Влево",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "Вправо",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "Caps Lock",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "Num Lock",
        ("key_scroll_lock", _) => "Scroll Lock",
        ("key_print_screen", _) => "Print Screen",
        ("key_pause", _) => "Пауза",
        ("key_menu", _) => "Меню",
        ("key_application", _) => "Приложение",

        ("key_shift_left", Textual) => "Левый Shift",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "Правый Shift",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "Левый Ctrl",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "Правый Ctrl",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "Левый Alt",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "Правый Alt",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "Левый Cmd",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "Правый Cmd",
        ("key_gui_right", Symbolic) => "◇",

        ("key_kp_enter", Textual) => "Ввод (Num)",
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

        ("mod_gui_mac", Textual) => "Command",
        ("mod_gui_mac", Symbolic) => "⌘",
        ("mod_gui_win", Textual) => "Windows",
        ("mod_gui_win", Symbolic) => "⊞",
        ("mod_gui_linux", Textual) => "Super",
        ("mod_gui_linux", Symbolic) => "◇",
        ("mod_gui_chromeos", Textual) => "Поиск",
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
