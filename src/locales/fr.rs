//! Français (`fr`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        ("key_escape", Textual) => "Échap",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "Entrée",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "Tab",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "Espace",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "Retour arrière",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "Inser",
        ("key_delete", Textual) => "Suppr",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", Textual) => "Origine",
        ("key_home", Symbolic) => "↖",
        ("key_end", Textual) => "Fin",
        ("key_end", Symbolic) => "↘",
        ("key_page_up", Textual) => "Page préc.",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "Page suiv.",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "Haut",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "Bas",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "Gauche",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "Droite",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "Verr. Maj.",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "Verr. Num.",
        ("key_scroll_lock", _) => "Arrêt défil.",
        ("key_print_screen", _) => "Impr. écran",
        ("key_pause", _) => "Pause",
        ("key_menu", _) => "Menu",
        ("key_application", _) => "Application",

        ("key_shift_left", Textual) => "Maj. gauche",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "Maj. droite",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "Ctrl gauche",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "Ctrl droit",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "Alt gauche",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "Alt droit",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "Cmd gauche",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "Cmd droite",
        ("key_gui_right", Symbolic) => "◇",

        // F-keys, keypad — mêmes étiquettes qu'en anglais.
        ("key_kp_enter", Textual) => "Entrée (pavé)",
        ("key_kp_enter", Symbolic) => "⏎",
        ("key_kp_divide", Textual) => "Pavé /",
        ("key_kp_divide", Symbolic) => "÷",
        ("key_kp_multiply", Textual) => "Pavé *",
        ("key_kp_multiply", Symbolic) => "×",
        ("key_kp_minus", Textual) => "Pavé -",
        ("key_kp_minus", Symbolic) => "−",
        ("key_kp_plus", Textual) => "Pavé +",
        ("key_kp_plus", Symbolic) => "+",
        ("key_kp_period", _) => "Pavé .",
        ("key_kp_equals", _) => "Pavé =",
        ("key_kp_0", _) => "Pavé 0",
        ("key_kp_1", _) => "Pavé 1",
        ("key_kp_2", _) => "Pavé 2",
        ("key_kp_3", _) => "Pavé 3",
        ("key_kp_4", _) => "Pavé 4",
        ("key_kp_5", _) => "Pavé 5",
        ("key_kp_6", _) => "Pavé 6",
        ("key_kp_7", _) => "Pavé 7",
        ("key_kp_8", _) => "Pavé 8",
        ("key_kp_9", _) => "Pavé 9",

        // Modifiers held — platform-aware.
        ("mod_ctrl_mac", Textual) => "Contrôle",
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
        | ("mod_shift", Textual) => "Maj.",

        ("mod_alt_mac", Textual) => "Option",
        ("mod_alt_mac", Symbolic) => "⌥",
        ("mod_alt_win", _)
        | ("mod_alt_linux", _)
        | ("mod_alt_chromeos", _)
        | ("mod_alt_android", _)
        | ("mod_alt", _) => "Alt",

        ("mod_gui_mac", Textual) => "Commande",
        ("mod_gui_mac", Symbolic) => "⌘",
        ("mod_gui_win", Textual) => "Windows",
        ("mod_gui_win", Symbolic) => "⊞",
        ("mod_gui_linux", Textual) => "Super",
        ("mod_gui_linux", Symbolic) => "◇",
        ("mod_gui_chromeos", Textual) => "Recherche",
        ("mod_gui_chromeos", Symbolic) => "◯",
        ("mod_gui_android", _) | ("mod_gui", _) => "Méta",

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
