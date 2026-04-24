//! हिन्दी (`hi`). Textual only — Symbolic glyphs fall back to English.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    if style != Textual {
        return None;
    }
    let s: &'static str = match key_id {
        "key_escape" => "एस्केप",
        "key_return" => "एंटर",
        "key_tab" => "टैब",
        "key_space" => "स्पेस",
        "key_backspace" => "बैकस्पेस",
        "key_insert" => "इंसर्ट",
        "key_delete" => "डिलीट",
        "key_home" => "होम",
        "key_end" => "एंड",
        "key_page_up" => "पेज ऊपर",
        "key_page_down" => "पेज नीचे",
        "key_arrow_up" => "ऊपर",
        "key_arrow_down" => "नीचे",
        "key_arrow_left" => "बायाँ",
        "key_arrow_right" => "दायाँ",
        "key_caps_lock" => "Caps Lock",
        "key_num_lock" => "Num Lock",
        "key_scroll_lock" => "Scroll Lock",
        "key_print_screen" => "प्रिंट स्क्रीन",
        "key_pause" => "रुकें",
        "key_menu" => "मेनू",
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
        "mod_gui_chromeos" => "खोजें",
        "mod_gui" | "mod_gui_android" => "Meta",
        "mod_altgr" | "mod_altgr_mac" | "mod_altgr_win" | "mod_altgr_linux"
        | "mod_altgr_chromeos" | "mod_altgr_android" => "AltGr",
        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
