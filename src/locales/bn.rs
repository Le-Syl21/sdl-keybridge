//! বাংলা (`bn`). Textual only — Symbolic glyphs fall back to English.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    if style != Textual {
        return None;
    }
    let s: &'static str = match key_id {
        "key_escape" => "এসকেপ",
        "key_return" => "এন্টার",
        "key_tab" => "ট্যাব",
        "key_space" => "স্পেস",
        "key_backspace" => "ব্যাকস্পেস",
        "key_insert" => "ইনসার্ট",
        "key_delete" => "ডিলিট",
        "key_home" => "হোম",
        "key_end" => "এন্ড",
        "key_page_up" => "পেজ আপ",
        "key_page_down" => "পেজ ডাউন",
        "key_arrow_up" => "উপর",
        "key_arrow_down" => "নীচ",
        "key_arrow_left" => "বাম",
        "key_arrow_right" => "ডান",
        "key_caps_lock" => "Caps Lock",
        "key_num_lock" => "Num Lock",
        "key_scroll_lock" => "Scroll Lock",
        "key_print_screen" => "প্রিন্ট স্ক্রিন",
        "key_pause" => "বিরতি",
        "key_menu" => "মেনু",
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
        "mod_gui_chromeos" => "অনুসন্ধান",
        "mod_gui" | "mod_gui_android" => "Meta",
        "mod_altgr" | "mod_altgr_mac" | "mod_altgr_win" | "mod_altgr_linux"
        | "mod_altgr_chromeos" | "mod_altgr_android" => "AltGr",
        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
