//! 한국어 (`ko`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        ("key_escape", Textual) => "Esc",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "엔터",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "탭",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "스페이스",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "백스페이스",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "삽입",
        ("key_delete", Textual) => "삭제",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", _) => "홈",
        ("key_end", _) => "엔드",
        ("key_page_up", Textual) => "페이지 위",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "페이지 아래",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "위",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "아래",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "왼쪽",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "오른쪽",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "Caps Lock",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "Num Lock",
        ("key_scroll_lock", _) => "Scroll Lock",
        ("key_print_screen", _) => "화면 인쇄",
        ("key_pause", _) => "일시 정지",
        ("key_menu", _) => "메뉴",
        ("key_application", _) => "애플리케이션",

        ("key_shift_left", Textual) => "왼쪽 Shift",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "오른쪽 Shift",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "왼쪽 Ctrl",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "오른쪽 Ctrl",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "왼쪽 Alt",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "오른쪽 Alt",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "왼쪽 Cmd",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "오른쪽 Cmd",
        ("key_gui_right", Symbolic) => "◇",

        ("key_kp_enter", Textual) => "엔터 (숫자 키패드)",
        ("key_kp_enter", Symbolic) => "⏎",
        ("key_kp_divide", Textual) => "숫자 키패드 /",
        ("key_kp_divide", Symbolic) => "÷",
        ("key_kp_multiply", Textual) => "숫자 키패드 *",
        ("key_kp_multiply", Symbolic) => "×",
        ("key_kp_minus", Textual) => "숫자 키패드 -",
        ("key_kp_minus", Symbolic) => "−",
        ("key_kp_plus", Textual) => "숫자 키패드 +",
        ("key_kp_plus", Symbolic) => "+",
        ("key_kp_period", _) => "숫자 키패드 .",
        ("key_kp_equals", _) => "숫자 키패드 =",
        ("key_kp_0", _) => "숫자 키패드 0",
        ("key_kp_1", _) => "숫자 키패드 1",
        ("key_kp_2", _) => "숫자 키패드 2",
        ("key_kp_3", _) => "숫자 키패드 3",
        ("key_kp_4", _) => "숫자 키패드 4",
        ("key_kp_5", _) => "숫자 키패드 5",
        ("key_kp_6", _) => "숫자 키패드 6",
        ("key_kp_7", _) => "숫자 키패드 7",
        ("key_kp_8", _) => "숫자 키패드 8",
        ("key_kp_9", _) => "숫자 키패드 9",

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
        ("mod_gui_chromeos", Textual) => "검색",
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
