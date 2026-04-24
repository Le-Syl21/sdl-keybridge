//! 繁體中文 (`zh-hant`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        ("key_escape", Textual) => "退出",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "輸入",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "定位",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "空格",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "退格",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "插入",
        ("key_delete", Textual) => "刪除",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", _) => "首頁",
        ("key_end", _) => "結束",
        ("key_page_up", Textual) => "上一頁",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "下一頁",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "上",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "下",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "左",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "右",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "大寫鎖定",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "數字鎖定",
        ("key_scroll_lock", _) => "捲動鎖定",
        ("key_print_screen", _) => "列印螢幕",
        ("key_pause", _) => "暫停",
        ("key_menu", _) => "選單",
        ("key_application", _) => "應用程式",

        ("key_shift_left", Textual) => "左 Shift",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "右 Shift",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "左 Ctrl",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "右 Ctrl",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "左 Alt",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "右 Alt",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "左 Cmd",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "右 Cmd",
        ("key_gui_right", Symbolic) => "◇",

        ("key_kp_enter", Textual) => "輸入 (數字鍵盤)",
        ("key_kp_enter", Symbolic) => "⏎",
        ("key_kp_divide", Textual) => "數字鍵盤 /",
        ("key_kp_divide", Symbolic) => "÷",
        ("key_kp_multiply", Textual) => "數字鍵盤 *",
        ("key_kp_multiply", Symbolic) => "×",
        ("key_kp_minus", Textual) => "數字鍵盤 -",
        ("key_kp_minus", Symbolic) => "−",
        ("key_kp_plus", Textual) => "數字鍵盤 +",
        ("key_kp_plus", Symbolic) => "+",
        ("key_kp_period", _) => "數字鍵盤 .",
        ("key_kp_equals", _) => "數字鍵盤 =",
        ("key_kp_0", _) => "數字鍵盤 0",
        ("key_kp_1", _) => "數字鍵盤 1",
        ("key_kp_2", _) => "數字鍵盤 2",
        ("key_kp_3", _) => "數字鍵盤 3",
        ("key_kp_4", _) => "數字鍵盤 4",
        ("key_kp_5", _) => "數字鍵盤 5",
        ("key_kp_6", _) => "數字鍵盤 6",
        ("key_kp_7", _) => "數字鍵盤 7",
        ("key_kp_8", _) => "數字鍵盤 8",
        ("key_kp_9", _) => "數字鍵盤 9",

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
        ("mod_gui_chromeos", Textual) => "搜尋",
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
