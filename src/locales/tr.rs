//! Türkçe (`tr`). Textual only — Symbolic glyphs fall back to English.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    if style != Textual {
        return None;
    }
    let s: &'static str = match key_id {
        "key_escape" => "Esc",
        "key_return" => "Giriş",
        "key_tab" => "Sekme",
        "key_space" => "Boşluk",
        "key_backspace" => "Geri",
        "key_insert" => "Ekle",
        "key_delete" => "Sil",
        "key_home" => "Başa",
        "key_end" => "Sona",
        "key_page_up" => "Önceki Sayfa",
        "key_page_down" => "Sonraki Sayfa",
        "key_arrow_up" => "Yukarı",
        "key_arrow_down" => "Aşağı",
        "key_arrow_left" => "Sol",
        "key_arrow_right" => "Sağ",
        "key_caps_lock" => "Büyük Harf Kilidi",
        "key_num_lock" => "Num Lock",
        "key_scroll_lock" => "Scroll Lock",
        "key_print_screen" => "Ekran Yazdır",
        "key_pause" => "Duraklat",
        "key_menu" => "Menü",
        "mod_ctrl" | "mod_ctrl_win" | "mod_ctrl_linux" | "mod_ctrl_chromeos"
        | "mod_ctrl_android" => "Ctrl",
        "mod_ctrl_mac" => "Kontrol",
        "mod_shift" | "mod_shift_mac" | "mod_shift_win" | "mod_shift_linux"
        | "mod_shift_chromeos" | "mod_shift_android" => "Üst Krkt",
        "mod_alt_mac" => "Seçenek",
        "mod_alt" | "mod_alt_win" | "mod_alt_linux" | "mod_alt_chromeos" | "mod_alt_android" => {
            "Alt"
        }
        "mod_gui_mac" => "Komut",
        "mod_gui_win" => "Windows",
        "mod_gui_linux" => "Süper",
        "mod_gui_chromeos" => "Ara",
        "mod_gui" | "mod_gui_android" => "Meta",
        "mod_altgr" | "mod_altgr_mac" | "mod_altgr_win" | "mod_altgr_linux"
        | "mod_altgr_chromeos" | "mod_altgr_android" => "AltGr",
        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
