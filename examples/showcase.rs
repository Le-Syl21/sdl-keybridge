//! Non-interactive walk-through of the four public API functions.
//!
//! Print-only — exercises eleven canonical scenarios in sequence so you
//! can eyeball the crate end-to-end without writing any driver code.
//! Use this to validate a build or to demo the crate to a teammate.
//!
//! For an *interactive* tool that takes a layout + scancode/keycode/name
//! on the command line, see `examples/inspect.rs`.
//!
//! ## Scenarios covered
//!
//! 1. Same physical key → different chars across QWERTY / AZERTY / QWERTZ / JCUKEN
//! 2. Named-key localization (Escape in en / fr / de / ja / ru)
//! 3. Symbolic vs Textual styles for arrows
//! 4. Caps Lock correctness — letters yes, digits/punctuation no
//! 5. NumLock toggling keypad between digit and navigation roles
//! 6. Platform-aware GUI / Alt modifier glyphs (`⌘` / `⊞` / `◇` / `◯`)
//! 7. French modifier labels on macOS
//! 8. Reverse `scancode_for` lookup
//! 9. Cross-layout binding bridge (Russian `ф` → French AZERTY `q`)
//! 10. `keycode_from_name` — parsing SDL textual key names
//! 11. User-side combo formatting (`"Ctrl+Maj.+Q"`)
//!
//! ## Run
//!
//! ```text
//! cargo run --example showcase --all-features
//! ```
//!
//! With only the default `en` feature, locale-specific lines fall back
//! to English (this is the intended fallback behavior).

use sdl_keybridge::{
    keycode_from_name, modifier_label, resolve, scancode_for, KeyMod, Keycode, LabelStyle,
    Modifier, MultiLocalizer, Platform, Scancode,
};

fn main() {
    let loc = MultiLocalizer::new();

    hr("1. resolve() — same physical key, four layouts");
    for (layout, locale) in [
        ("windows/en-t-k0-windows", "en"),
        ("windows/fr-t-k0-windows", "fr"),
        ("windows/de-t-k0-windows", "de"),
        ("windows/ru-t-k0-windows", "ru"),
    ] {
        let r = resolve(
            Scancode::Q,
            KeyMod::NONE,
            layout,
            locale,
            LabelStyle::Textual,
            &loc,
        );
        println!(
            "  Scancode::Q on {layout:<30} → char={:?} keycode=0x{:08x} glyph_local={:?}",
            r.character,
            r.keycode.raw(),
            r.glyph_local
        );
    }

    hr("2. resolve() — named key across locales (Textual)");
    for (locale, label) in [
        ("en", "English"),
        ("fr", "Français"),
        ("de", "Deutsch"),
        ("ja", "日本語"),
        ("ru", "Русский"),
    ] {
        let r = resolve(
            Scancode::ESCAPE,
            KeyMod::NONE,
            "windows/en-t-k0-windows",
            locale,
            LabelStyle::Textual,
            &loc,
        );
        println!("  Escape in {label:<10} → {}", r.glyph_local);
    }

    hr("3. resolve() — Symbolic vs Textual for arrows");
    for sc in [
        Scancode::UP,
        Scancode::DOWN,
        Scancode::LEFT,
        Scancode::RIGHT,
    ] {
        let sym = resolve(
            sc,
            KeyMod::NONE,
            "windows/en-t-k0-windows",
            "en",
            LabelStyle::Symbolic,
            &loc,
        );
        let txt = resolve(
            sc,
            KeyMod::NONE,
            "windows/en-t-k0-windows",
            "en",
            LabelStyle::Textual,
            &loc,
        );
        println!(
            "  scancode={:3} → Symbolic={:<4} Textual={}",
            sc.raw(),
            sym.glyph_local,
            txt.glyph_local
        );
    }

    hr("4. resolve() — Caps Lock handling (letters only)");
    let layouts = "windows/en-t-k0-windows";
    for (sc, label) in [
        (Scancode::A, "letter 'a'"),
        (Scancode::NUM_1, "digit '1'"),
        (Scancode::LEFT_BRACKET, "bracket '['"),
    ] {
        let r = resolve(sc, KeyMod::CAPS, layouts, "en", LabelStyle::Textual, &loc);
        println!("  Caps + {label:<14} → char={:?}", r.character);
    }

    hr("5. resolve() — NumLock OFF keypad = navigation");
    for (sc, role) in [
        (Scancode::KP_1, "Keypad 1"),
        (Scancode::KP_2, "Keypad 2"),
        (Scancode::KP_7, "Keypad 7"),
        (Scancode::KP_PERIOD, "Keypad ."),
    ] {
        let on = resolve(
            sc,
            KeyMod::NUM,
            "windows/en-t-k0-windows",
            "en",
            LabelStyle::Textual,
            &loc,
        );
        let off = resolve(
            sc,
            KeyMod::NONE,
            "windows/en-t-k0-windows",
            "en",
            LabelStyle::Textual,
            &loc,
        );
        println!(
            "  {role:<10} → NumLock ON={:<5} OFF={:?}",
            format!("{:?}", on.character),
            off.named_key
        );
    }

    hr("6. modifier_label() — platform-aware (Textual + Symbolic)");
    for plat in [
        Platform::Mac,
        Platform::Windows,
        Platform::Linux,
        Platform::ChromeOS,
    ] {
        let gui_t = modifier_label(Modifier::Gui, plat, "en", LabelStyle::Textual, &loc);
        let gui_s = modifier_label(Modifier::Gui, plat, "en", LabelStyle::Symbolic, &loc);
        let alt_t = modifier_label(Modifier::Alt, plat, "en", LabelStyle::Textual, &loc);
        let alt_s = modifier_label(Modifier::Alt, plat, "en", LabelStyle::Symbolic, &loc);
        println!("  {plat:<10?} → Gui = {gui_t}/{gui_s}   Alt = {alt_t}/{alt_s}");
    }

    hr("7. modifier_label() — French on macOS");
    for m in [
        Modifier::Ctrl,
        Modifier::Shift,
        Modifier::Alt,
        Modifier::Gui,
        Modifier::AltGr,
    ] {
        let t = modifier_label(m, Platform::Mac, "fr", LabelStyle::Textual, &loc);
        let s = modifier_label(m, Platform::Mac, "fr", LabelStyle::Symbolic, &loc);
        println!("  {m:<7?} → {t} / {s}");
    }

    hr("8. scancode_for() — reverse lookup");
    for (kc, layout) in [
        (Keycode::from('a'), "windows/en-t-k0-windows"),
        (Keycode::from('q'), "windows/fr-t-k0-windows"),
        (Keycode::from('ф'), "windows/ru-t-k0-windows"),
        (Keycode::F5, "windows/en-t-k0-windows"),
    ] {
        let sc = scancode_for(kc, layout);
        println!("  keycode=0x{:08x} in {layout:<30} → {:?}", kc.raw(), sc);
    }

    hr("9. Cross-layout bridge — Russian binding replayed on AZERTY");
    // User recorded a binding on "ф" (ru-jcuken). What does the physical
    // key produce on a French AZERTY keyboard?
    let sc = scancode_for(Keycode::from('ф'), "windows/ru-t-k0-windows").unwrap();
    let r = resolve(
        sc,
        KeyMod::NONE,
        "windows/fr-t-k0-windows",
        "fr",
        LabelStyle::Textual,
        &loc,
    );
    println!(
        "  ф (Russian) ≡ scancode {:>3} ≡ {:?} (French AZERTY)",
        sc.raw(),
        r.character
    );

    hr("10. keycode_from_name() — inverse of SDL_GetKeyName");
    for name in [
        "Escape",
        "Left Shift",
        "F5",
        "Page Up",
        "a",
        "Keypad 7",
        "nonsense",
    ] {
        println!(
            "  {name:<12} → {:?}",
            keycode_from_name(name).map(|k| format!("0x{:08x}", k.raw()))
        );
    }

    hr("11. Combo formatting — user assembles the labels themselves");
    let ctrl = modifier_label(
        Modifier::Ctrl,
        Platform::Linux,
        "fr",
        LabelStyle::Textual,
        &loc,
    );
    let shift = modifier_label(
        Modifier::Shift,
        Platform::Linux,
        "fr",
        LabelStyle::Textual,
        &loc,
    );
    let key = resolve(
        Scancode::A,
        KeyMod::LCTRL | KeyMod::LSHIFT,
        "windows/fr-t-k0-windows",
        "fr",
        LabelStyle::Textual,
        &loc,
    );
    println!(
        "  Ctrl+Shift+A (AZERTY) → \"{ctrl}+{shift}+{}\"",
        key.character.unwrap_or('?')
    );
}

fn hr(title: &str) {
    println!("\n═══ {title} ═══");
}
