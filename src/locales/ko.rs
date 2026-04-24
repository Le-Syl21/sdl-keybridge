//! 한국어 (`ko`). Textual only — Symbolic glyphs fall back to English.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    if style != Textual {
        return None;
    }
    let s: &'static str = match key_id {
        "key_escape" => "Esc",
        "key_return" => "엔터",
        "key_tab" => "탭",
        "key_space" => "스페이스",
        "key_backspace" => "백스페이스",
        "key_insert" => "삽입",
        "key_delete" => "삭제",
        "key_home" => "홈",
        "key_end" => "엔드",
        "key_page_up" => "페이지 위",
        "key_page_down" => "페이지 아래",
        "key_arrow_up" => "위",
        "key_arrow_down" => "아래",
        "key_arrow_left" => "왼쪽",
        "key_arrow_right" => "오른쪽",
        "key_caps_lock" => "Caps Lock",
        "key_num_lock" => "Num Lock",
        "key_scroll_lock" => "Scroll Lock",
        "key_print_screen" => "화면 인쇄",
        "key_pause" => "일시 정지",
        "key_menu" => "메뉴",
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
        "mod_gui_chromeos" => "검색",
        "mod_gui" | "mod_gui_android" => "Meta",
        "mod_altgr" | "mod_altgr_mac" | "mod_altgr_win" | "mod_altgr_linux"
        | "mod_altgr_chromeos" | "mod_altgr_android" => "AltGr",
        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
