//! ภาษาไทย (`th`). Textual only — Symbolic glyphs fall back to English.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    if style != Textual {
        return None;
    }
    let s: &'static str = match key_id {
        "key_escape" => "Esc",
        "key_return" => "Enter",
        "key_tab" => "Tab",
        "key_space" => "เว้นวรรค",
        "key_backspace" => "ลบย้อนกลับ",
        "key_insert" => "แทรก",
        "key_delete" => "ลบ",
        "key_home" => "หน้าแรก",
        "key_end" => "ท้าย",
        "key_page_up" => "เลื่อนหน้าขึ้น",
        "key_page_down" => "เลื่อนหน้าลง",
        "key_arrow_up" => "ขึ้น",
        "key_arrow_down" => "ลง",
        "key_arrow_left" => "ซ้าย",
        "key_arrow_right" => "ขวา",
        "key_caps_lock" => "Caps Lock",
        "key_num_lock" => "Num Lock",
        "key_scroll_lock" => "Scroll Lock",
        "key_print_screen" => "พิมพ์หน้าจอ",
        "key_pause" => "หยุดชั่วคราว",
        "key_menu" => "เมนู",
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
        "mod_gui_chromeos" => "ค้นหา",
        "mod_gui" | "mod_gui_android" => "Meta",
        "mod_altgr" | "mod_altgr_mac" | "mod_altgr_win" | "mod_altgr_linux"
        | "mod_altgr_chromeos" | "mod_altgr_android" => "AltGr",
        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
