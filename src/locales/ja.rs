//! 日本語 (`ja`). Textual only — Symbolic glyphs fall back to English.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    if style != Textual {
        return None;
    }
    let s: &'static str = match key_id {
        "key_escape" => "エスケープ",
        "key_return" => "エンター",
        "key_tab" => "タブ",
        "key_space" => "スペース",
        "key_backspace" => "バックスペース",
        "key_insert" => "インサート",
        "key_delete" => "デリート",
        "key_home" => "ホーム",
        "key_end" => "エンド",
        "key_page_up" => "ページアップ",
        "key_page_down" => "ページダウン",
        "key_arrow_up" => "上",
        "key_arrow_down" => "下",
        "key_arrow_left" => "左",
        "key_arrow_right" => "右",
        "key_caps_lock" => "キャプスロック",
        "key_num_lock" => "ナムロック",
        "key_scroll_lock" => "スクロールロック",
        "key_print_screen" => "プリントスクリーン",
        "key_pause" => "ポーズ",
        "key_menu" => "メニュー",
        "mod_ctrl" | "mod_ctrl_win" | "mod_ctrl_linux" | "mod_ctrl_chromeos"
        | "mod_ctrl_android" => "Ctrl",
        "mod_ctrl_mac" => "コントロール",
        "mod_shift" | "mod_shift_mac" | "mod_shift_win" | "mod_shift_linux"
        | "mod_shift_chromeos" | "mod_shift_android" => "シフト",
        "mod_alt_mac" => "オプション",
        "mod_alt" | "mod_alt_win" | "mod_alt_linux" | "mod_alt_chromeos" | "mod_alt_android" => {
            "Alt"
        }
        "mod_gui_mac" => "コマンド",
        "mod_gui_win" => "Windows",
        "mod_gui_linux" => "Super",
        "mod_gui_chromeos" => "検索",
        "mod_gui" | "mod_gui_android" => "Meta",
        "mod_altgr" | "mod_altgr_mac" | "mod_altgr_win" | "mod_altgr_linux"
        | "mod_altgr_chromeos" | "mod_altgr_android" => "AltGr",
        _ => return None,
    };
    Some(Cow::Borrowed(s))
}
