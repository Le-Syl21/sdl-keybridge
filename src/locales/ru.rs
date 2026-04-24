//! Русский (`ru`). Textual only — Symbolic glyphs fall back to English.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    if style != Textual {
        return None;
    }
    let s: &'static str = match key_id {
        "key_escape" => "Esc",
        "key_return" => "Ввод",
        "key_tab" => "Tab",
        "key_space" => "Пробел",
        "key_backspace" => "Backspace",
        "key_insert" => "Вставка",
        "key_delete" => "Удал",
        "key_home" => "Home",
        "key_end" => "End",
        "key_page_up" => "Страница вверх",
        "key_page_down" => "Страница вниз",
        "key_arrow_up" => "Вверх",
        "key_arrow_down" => "Вниз",
        "key_arrow_left" => "Влево",
        "key_arrow_right" => "Вправо",
        "key_caps_lock" => "Caps Lock",
        "key_num_lock" => "Num Lock",
        "key_scroll_lock" => "Scroll Lock",
        "key_print_screen" => "Print Screen",
        "key_pause" => "Пауза",
        "key_menu" => "Меню",
        "mod_ctrl" | "mod_ctrl_win" | "mod_ctrl_linux" | "mod_ctrl_chromeos"
        | "mod_ctrl_android" => "Ctrl",
        "mod_ctrl_mac" => "Control",
        "mod_shift" | "mod_shift_mac" | "mod_shift_win" | "mod_shift_linux"
        | "mod_shift_chromeos" | "mod_shift_android" => "Shift",
        "mod_alt_mac" => "Option",
        "mod_alt" | "mod_alt_win" | "mod_alt_linux" | "mod_alt_chromeos" | "mod_alt_android" => {
            "Alt"
        }
        "mod_gui_mac" => "Command",
        "mod_gui_win" => "Windows",
        "mod_gui_linux" => "Super",
        "mod_gui_chromeos" => "Поиск",
        "mod_gui" | "mod_gui_android" => "Meta",
        "mod_altgr" | "mod_altgr_mac" | "mod_altgr_win" | "mod_altgr_linux"
        | "mod_altgr_chromeos" | "mod_altgr_android" => "AltGr",
        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
