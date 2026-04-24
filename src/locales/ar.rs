//! العربية (`ar`). Textual only — Symbolic glyphs fall back to English.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    if style != Textual {
        return None;
    }
    let s: &'static str = match key_id {
        "key_escape" => "خروج",
        "key_return" => "إدخال",
        "key_tab" => "جدولة",
        "key_space" => "مسافة",
        "key_backspace" => "مسح للخلف",
        "key_insert" => "إدراج",
        "key_delete" => "حذف",
        "key_home" => "الصفحة الرئيسية",
        "key_end" => "نهاية",
        "key_page_up" => "صفحة للأعلى",
        "key_page_down" => "صفحة للأسفل",
        "key_arrow_up" => "أعلى",
        "key_arrow_down" => "أسفل",
        "key_arrow_left" => "يسار",
        "key_arrow_right" => "يمين",
        "key_caps_lock" => "Caps Lock",
        "key_num_lock" => "Num Lock",
        "key_scroll_lock" => "Scroll Lock",
        "key_print_screen" => "طباعة الشاشة",
        "key_pause" => "إيقاف مؤقت",
        "key_menu" => "قائمة",
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
        "mod_gui_chromeos" => "بحث",
        "mod_gui" | "mod_gui_android" => "Meta",
        "mod_altgr" | "mod_altgr_mac" | "mod_altgr_win" | "mod_altgr_linux"
        | "mod_altgr_chromeos" | "mod_altgr_android" => "AltGr",
        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
