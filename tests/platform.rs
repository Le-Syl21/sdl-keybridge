//! Platform-aware modifier labels. These assert English strings so they
//! require the `en` feature.

#![cfg(feature = "en")]

use sdl_keybridge::{modifier_label, LabelStyle, Modifier, MultiLocalizer, Platform};

#[test]
fn gui_modifier_per_platform_symbolic() {
    let loc = MultiLocalizer::new();
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Mac,
            "en",
            LabelStyle::Symbolic,
            &loc
        )
        .as_ref(),
        "⌘"
    );
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Windows,
            "en",
            LabelStyle::Symbolic,
            &loc
        )
        .as_ref(),
        "⊞"
    );
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Linux,
            "en",
            LabelStyle::Symbolic,
            &loc
        )
        .as_ref(),
        "◇"
    );
}

#[test]
fn gui_modifier_per_platform_textual() {
    let loc = MultiLocalizer::new();
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Mac,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Command"
    );
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Windows,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Windows"
    );
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::Linux,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Super"
    );
    assert_eq!(
        modifier_label(
            Modifier::Gui,
            Platform::ChromeOS,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Search"
    );
}

#[test]
fn alt_modifier_per_platform() {
    let loc = MultiLocalizer::new();
    assert_eq!(
        modifier_label(
            Modifier::Alt,
            Platform::Mac,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Option"
    );
    assert_eq!(
        modifier_label(
            Modifier::Alt,
            Platform::Windows,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Alt"
    );
    assert_eq!(
        modifier_label(
            Modifier::Alt,
            Platform::Mac,
            "en",
            LabelStyle::Symbolic,
            &loc
        )
        .as_ref(),
        "⌥"
    );
}

#[test]
fn ctrl_modifier_textual_distinguishes_mac() {
    let loc = MultiLocalizer::new();
    // On macOS, "Control" is spelled out; elsewhere it is "Ctrl".
    assert_eq!(
        modifier_label(
            Modifier::Ctrl,
            Platform::Mac,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Control"
    );
    assert_eq!(
        modifier_label(
            Modifier::Ctrl,
            Platform::Windows,
            "en",
            LabelStyle::Textual,
            &loc
        )
        .as_ref(),
        "Ctrl"
    );
}
