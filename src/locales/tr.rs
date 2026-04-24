//! Türkçe (`tr`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        ("key_escape", Textual) => "Esc",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "Giriş",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "Sekme",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "Boşluk",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "Geri",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "Ekle",
        ("key_delete", Textual) => "Sil",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", _) => "Başa",
        ("key_end", _) => "Sona",
        ("key_page_up", Textual) => "Önceki Sayfa",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "Sonraki Sayfa",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "Yukarı",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "Aşağı",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "Sol",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "Sağ",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "Büyük Harf Kilidi",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "Num Lock",
        ("key_scroll_lock", _) => "Scroll Lock",
        ("key_print_screen", _) => "Ekran Yazdır",
        ("key_pause", _) => "Duraklat",
        ("key_menu", _) => "Menü",
        ("key_application", _) => "Uygulama",

        ("key_shift_left", Textual) => "Sol Shift",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "Sağ Shift",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "Sol Ctrl",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "Sağ Ctrl",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "Sol Alt",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "Sağ Alt",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "Sol Cmd",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "Sağ Cmd",
        ("key_gui_right", Symbolic) => "◇",

        ("key_kp_enter", Textual) => "Giriş (Num)",
        ("key_kp_enter", Symbolic) => "⏎",
        ("key_kp_divide", Textual) => "Num /",
        ("key_kp_divide", Symbolic) => "÷",
        ("key_kp_multiply", Textual) => "Num *",
        ("key_kp_multiply", Symbolic) => "×",
        ("key_kp_minus", Textual) => "Num -",
        ("key_kp_minus", Symbolic) => "−",
        ("key_kp_plus", Textual) => "Num +",
        ("key_kp_plus", Symbolic) => "+",
        ("key_kp_period", _) => "Num .",
        ("key_kp_equals", _) => "Num =",
        ("key_kp_0", _) => "Num 0",
        ("key_kp_1", _) => "Num 1",
        ("key_kp_2", _) => "Num 2",
        ("key_kp_3", _) => "Num 3",
        ("key_kp_4", _) => "Num 4",
        ("key_kp_5", _) => "Num 5",
        ("key_kp_6", _) => "Num 6",
        ("key_kp_7", _) => "Num 7",
        ("key_kp_8", _) => "Num 8",
        ("key_kp_9", _) => "Num 9",

        ("mod_ctrl_mac", Textual) => "Kontrol",
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
        | ("mod_shift", Textual) => "Üst Krkt",

        ("mod_alt_mac", Textual) => "Seçenek",
        ("mod_alt_mac", Symbolic) => "⌥",
        ("mod_alt_win", _)
        | ("mod_alt_linux", _)
        | ("mod_alt_chromeos", _)
        | ("mod_alt_android", _)
        | ("mod_alt", _) => "Alt",

        ("mod_gui_mac", Textual) => "Komut",
        ("mod_gui_mac", Symbolic) => "⌘",
        ("mod_gui_win", Textual) => "Windows",
        ("mod_gui_win", Symbolic) => "⊞",
        ("mod_gui_linux", Textual) => "Süper",
        ("mod_gui_linux", Symbolic) => "◇",
        ("mod_gui_chromeos", Textual) => "Ara",
        ("mod_gui_chromeos", Symbolic) => "◯",
        ("mod_gui_android", _) | ("mod_gui", _) => "Meta",

        ("mod_altgr_mac", Textual) => "Seçenek",
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
