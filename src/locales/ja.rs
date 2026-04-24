//! 日本語 (`ja`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        ("key_escape", Textual) => "エスケープ",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "エンター",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "タブ",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "スペース",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "バックスペース",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "インサート",
        ("key_delete", Textual) => "デリート",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", _) => "ホーム",
        ("key_end", _) => "エンド",
        ("key_page_up", Textual) => "ページアップ",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "ページダウン",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "上",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "下",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "左",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "右",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "キャプスロック",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "ナムロック",
        ("key_scroll_lock", _) => "スクロールロック",
        ("key_print_screen", _) => "プリントスクリーン",
        ("key_pause", _) => "ポーズ",
        ("key_menu", _) => "メニュー",
        ("key_application", _) => "アプリケーション",

        ("key_shift_left", Textual) => "左シフト",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "右シフト",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "左コントロール",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "右コントロール",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "左オプション",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "右オプション",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "左コマンド",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "右コマンド",
        ("key_gui_right", Symbolic) => "◇",

        ("key_kp_enter", Textual) => "エンター(テンキー)",
        ("key_kp_enter", Symbolic) => "⏎",
        ("key_kp_divide", Textual) => "テンキー /",
        ("key_kp_divide", Symbolic) => "÷",
        ("key_kp_multiply", Textual) => "テンキー *",
        ("key_kp_multiply", Symbolic) => "×",
        ("key_kp_minus", Textual) => "テンキー -",
        ("key_kp_minus", Symbolic) => "−",
        ("key_kp_plus", Textual) => "テンキー +",
        ("key_kp_plus", Symbolic) => "+",
        ("key_kp_period", _) => "テンキー .",
        ("key_kp_equals", _) => "テンキー =",
        ("key_kp_0", _) => "テンキー 0",
        ("key_kp_1", _) => "テンキー 1",
        ("key_kp_2", _) => "テンキー 2",
        ("key_kp_3", _) => "テンキー 3",
        ("key_kp_4", _) => "テンキー 4",
        ("key_kp_5", _) => "テンキー 5",
        ("key_kp_6", _) => "テンキー 6",
        ("key_kp_7", _) => "テンキー 7",
        ("key_kp_8", _) => "テンキー 8",
        ("key_kp_9", _) => "テンキー 9",

        ("mod_ctrl_mac", Textual) => "コントロール",
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
        | ("mod_shift", Textual) => "シフト",

        ("mod_alt_mac", Textual) => "オプション",
        ("mod_alt_mac", Symbolic) => "⌥",
        ("mod_alt_win", _)
        | ("mod_alt_linux", _)
        | ("mod_alt_chromeos", _)
        | ("mod_alt_android", _)
        | ("mod_alt", _) => "Alt",

        ("mod_gui_mac", Textual) => "コマンド",
        ("mod_gui_mac", Symbolic) => "⌘",
        ("mod_gui_win", Textual) => "Windows",
        ("mod_gui_win", Symbolic) => "⊞",
        ("mod_gui_linux", Textual) => "Super",
        ("mod_gui_linux", Symbolic) => "◇",
        ("mod_gui_chromeos", Textual) => "検索",
        ("mod_gui_chromeos", Symbolic) => "◯",
        ("mod_gui_android", _) | ("mod_gui", _) => "Meta",

        ("mod_altgr_mac", Textual) => "オプション",
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
