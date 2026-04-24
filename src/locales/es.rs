//! Español (`es`).

use std::borrow::Cow;

use crate::localizer::LabelStyle;

pub(crate) fn translate(key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>> {
    use LabelStyle::*;
    let s: &'static str = match (key_id, style) {
        // Editing / navigation keys.
        ("key_escape", Textual) => "Esc",
        ("key_escape", Symbolic) => "⎋",
        ("key_return", Textual) => "Intro",
        ("key_return", Symbolic) => "⏎",
        ("key_tab", Textual) => "Tab",
        ("key_tab", Symbolic) => "⇥",
        ("key_space", Textual) => "Espacio",
        ("key_space", Symbolic) => "␣",
        ("key_backspace", Textual) => "Retroceso",
        ("key_backspace", Symbolic) => "⌫",
        ("key_insert", _) => "Insertar",
        ("key_delete", Textual) => "Supr",
        ("key_delete", Symbolic) => "⌦",
        ("key_home", Textual) => "Inicio",
        ("key_home", Symbolic) => "↖",
        ("key_end", Textual) => "Fin",
        ("key_end", Symbolic) => "↘",
        ("key_page_up", Textual) => "Re Pág",
        ("key_page_up", Symbolic) => "⇞",
        ("key_page_down", Textual) => "Av Pág",
        ("key_page_down", Symbolic) => "⇟",

        ("key_arrow_up", Textual) => "Arriba",
        ("key_arrow_up", Symbolic) => "↑",
        ("key_arrow_down", Textual) => "Abajo",
        ("key_arrow_down", Symbolic) => "↓",
        ("key_arrow_left", Textual) => "Izquierda",
        ("key_arrow_left", Symbolic) => "←",
        ("key_arrow_right", Textual) => "Derecha",
        ("key_arrow_right", Symbolic) => "→",

        ("key_caps_lock", Textual) => "Bloq Mayús",
        ("key_caps_lock", Symbolic) => "⇪",
        ("key_num_lock", _) => "Bloq Num",
        ("key_scroll_lock", _) => "Bloq Despl",
        ("key_print_screen", _) => "Impr Pant",
        ("key_pause", _) => "Pausa",
        ("key_menu", _) => "Menú",
        ("key_application", _) => "Aplicación",

        // Modifier keys (as keys, not as held-modifier labels).
        ("key_shift_left", Textual) => "Mayús izq.",
        ("key_shift_left", Symbolic) => "⇧",
        ("key_shift_right", Textual) => "Mayús der.",
        ("key_shift_right", Symbolic) => "⇧",
        ("key_control_left", Textual) => "Ctrl izq.",
        ("key_control_left", Symbolic) => "⌃",
        ("key_control_right", Textual) => "Ctrl der.",
        ("key_control_right", Symbolic) => "⌃",
        ("key_alt_left", Textual) => "Alt izq.",
        ("key_alt_left", Symbolic) => "⌥",
        ("key_alt_right", Textual) => "Alt der.",
        ("key_alt_right", Symbolic) => "⌥",
        ("key_altgr", Textual) => "AltGr",
        ("key_altgr", Symbolic) => "⌥",
        ("key_gui_left", Textual) => "Cmd izq.",
        ("key_gui_left", Symbolic) => "◇",
        ("key_gui_right", Textual) => "Cmd der.",
        ("key_gui_right", Symbolic) => "◇",

        // Keypad keys. `Núm.` is the standard Microsoft Spanish abbreviation.
        ("key_kp_enter", Textual) => "Intro (Núm.)",
        ("key_kp_enter", Symbolic) => "⏎",
        ("key_kp_divide", Textual) => "Núm. /",
        ("key_kp_divide", Symbolic) => "÷",
        ("key_kp_multiply", Textual) => "Núm. *",
        ("key_kp_multiply", Symbolic) => "×",
        ("key_kp_minus", Textual) => "Núm. -",
        ("key_kp_minus", Symbolic) => "−",
        ("key_kp_plus", Textual) => "Núm. +",
        ("key_kp_plus", Symbolic) => "+",
        ("key_kp_period", _) => "Núm. .",
        ("key_kp_equals", _) => "Núm. =",
        ("key_kp_0", _) => "Núm. 0",
        ("key_kp_1", _) => "Núm. 1",
        ("key_kp_2", _) => "Núm. 2",
        ("key_kp_3", _) => "Núm. 3",
        ("key_kp_4", _) => "Núm. 4",
        ("key_kp_5", _) => "Núm. 5",
        ("key_kp_6", _) => "Núm. 6",
        ("key_kp_7", _) => "Núm. 7",
        ("key_kp_8", _) => "Núm. 8",
        ("key_kp_9", _) => "Núm. 9",

        // Modifier labels — platform-aware.
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
        | ("mod_shift", Textual) => "Mayús",

        ("mod_alt_mac", Textual) => "Opción",
        ("mod_alt_mac", Symbolic) => "⌥",
        ("mod_alt_win", _)
        | ("mod_alt_linux", _)
        | ("mod_alt_chromeos", _)
        | ("mod_alt_android", _)
        | ("mod_alt", _) => "Alt",

        ("mod_gui_mac", Textual) => "Comando",
        ("mod_gui_mac", Symbolic) => "⌘",
        ("mod_gui_win", Textual) => "Windows",
        ("mod_gui_win", Symbolic) => "⊞",
        ("mod_gui_linux", Textual) => "Super",
        ("mod_gui_linux", Symbolic) => "◇",
        ("mod_gui_chromeos", Textual) => "Buscar",
        ("mod_gui_chromeos", Symbolic) => "◯",
        ("mod_gui_android", _) | ("mod_gui", _) => "Meta",

        ("mod_altgr_mac", Textual) => "Opción",
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
