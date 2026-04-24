//! 简体中文 (`zh-hans`). Textual only — Symbolic glyphs fall back to English.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    if style != Textual {
        return None;
    }
    let s: &'static str = match key_id {
        "key_escape" => "退出",
        "key_return" => "回车",
        "key_tab" => "制表",
        "key_space" => "空格",
        "key_backspace" => "退格",
        "key_insert" => "插入",
        "key_delete" => "删除",
        "key_home" => "起始",
        "key_end" => "结束",
        "key_page_up" => "上一页",
        "key_page_down" => "下一页",
        "key_arrow_up" => "上",
        "key_arrow_down" => "下",
        "key_arrow_left" => "左",
        "key_arrow_right" => "右",
        "key_caps_lock" => "大写锁定",
        "key_num_lock" => "数字锁定",
        "key_scroll_lock" => "滚动锁定",
        "key_print_screen" => "打印屏幕",
        "key_pause" => "暂停",
        "key_menu" => "菜单",
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
        "mod_gui_chromeos" => "搜索",
        "mod_gui" | "mod_gui_android" => "Meta",
        "mod_altgr" | "mod_altgr_mac" | "mod_altgr_win" | "mod_altgr_linux"
        | "mod_altgr_chromeos" | "mod_altgr_android" => "AltGr",
        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
