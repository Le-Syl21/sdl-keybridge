//! Interactive CLI inspector — feed it a layout + an input key and it
//! dumps every parallel representation the crate can compute.
//!
//! Use this as the manual exploration harness to validate a layout
//! entry, debug a locale, or eyeball the cross-layout bridge by hand.
//! No GUI demo is planned — this CLI covers that role.
//!
//! ## Synopsis
//!
//! ```text
//! cargo run --example inspect --all-features -- \
//!     <layout> <kind> <value> [mods] [locale] [platform]
//! ```
//!
//! | Arg          | Meaning |
//! |--------------|---------|
//! | `<layout>`   | layout id, e.g. `linux/fr-t-k0-azerty`. Pass `?` to list every layout in this build. |
//! | `<kind>`     | one of `scancode` \| `keycode` \| `name`. |
//! | `<value>`    | for `scancode` / `keycode`: a decimal or `0x…` integer. For `name`: a textual SDL key name (`"Escape"`, `"Left Shift"`, `"F5"`, `"Keypad 7"`, `"a"`, …). |
//! | `[mods]`     | optional comma-separated subset of `shift`, `ctrl`, `alt`, `gui`, `altgr`, `caps`, `num` (left-side variants by default; suffix with `r`/`l` for the other side). Use `''` to mean "no modifiers". |
//! | `[locale]`   | optional BCP 47 tag (default: `en`). |
//! | `[platform]` | optional `mac` \| `windows` \| `linux` \| `chromeos` \| `android` (default: `linux`). |
//!
//! ## Output sections
//!
//! - **Resolved [Textual]** + **Resolved [Symbolic]** — the full
//!   [`Resolved`](sdl_keybridge::Resolved) struct in both styles.
//! - **Layout glyphs at every modifier level** — the four-level
//!   base / shift / altgr / shift+altgr table for printable keys.
//! - **modifier_label() on `<platform>` for locale `<locale>`** — every
//!   [`Modifier`](sdl_keybridge::Modifier) labeled both ways.
//! - **Same physical key replayed on every layout** — cross-layout
//!   bridge demo: shows what the same scancode produces on the other
//!   14 layouts shipped in the build.
//!
//! ## Examples
//!
//! ```text
//! # Letter A scancode on every layout, with Shift, French UI
//! cargo run --example inspect --all-features -- \
//!     linux/fr-t-k0-azerty scancode 4 shift fr linux
//!
//! # Inspect what Shift+2 produces on macOS German QWERTZ (should be ")
//! cargo run --example inspect --all-features -- \
//!     mac/de-t-k0-qwertz scancode 31 shift de mac
//!
//! # Look up Escape by its keycode
//! cargo run --example inspect --all-features -- \
//!     linux/en-US-t-k0-qwerty keycode 0x40000029
//!
//! # Look up Left Shift by SDL textual name
//! cargo run --example inspect --all-features -- \
//!     linux/en-US-t-k0-qwerty name "Left Shift"
//!
//! # List every layout in the current build
//! cargo run --example inspect --all-features -- ? scancode 0
//! ```

use std::env;
use std::process::ExitCode;

use sdl_keybridge::{
    all_layouts, get_layout, keycode_from_name, modifier_label, resolve, scancode_for, KeyMod,
    Keycode, LabelStyle, Modifier, MultiLocalizer, Platform, Scancode,
};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        usage();
        return ExitCode::from(2);
    }

    let layout_arg = &args[0];
    let kind = args[1].as_str();
    let value = args[2].as_str();
    let mods_arg = args.get(3).map(String::as_str).unwrap_or("");
    let locale = args.get(4).map(String::as_str).unwrap_or("en");
    let platform_arg = args.get(5).map(String::as_str).unwrap_or("linux");

    if layout_arg == "?" {
        println!("Layouts compiled into this build:");
        for l in all_layouts() {
            println!("  {:<35}  {}", l.id, l.display_name);
        }
        return ExitCode::SUCCESS;
    }

    let Some(layout) = get_layout(layout_arg) else {
        eprintln!("error: unknown layout id {layout_arg:?}");
        eprintln!("       try `cargo run --example inspect -- ? scancode 0` to list them.");
        return ExitCode::from(1);
    };

    let mods = match parse_mods(mods_arg) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: bad modifier list: {e}");
            return ExitCode::from(1);
        }
    };

    let platform = match platform_arg {
        "mac" => Platform::Mac,
        "windows" | "win" => Platform::Windows,
        "linux" => Platform::Linux,
        "chromeos" => Platform::ChromeOS,
        "android" => Platform::Android,
        _ => {
            eprintln!("error: unknown platform {platform_arg:?}");
            return ExitCode::from(1);
        }
    };

    // Resolve the input down to a scancode (the canonical pivot).
    let scancode: Scancode = match kind {
        "scancode" => match parse_int(value) {
            Some(v) => Scancode::new(v),
            None => {
                eprintln!("error: bad scancode value {value:?}");
                return ExitCode::from(1);
            }
        },
        "keycode" => match parse_int(value) {
            Some(v) => match scancode_for(Keycode::new(v), layout.id) {
                Some(sc) => sc,
                None => {
                    eprintln!(
                        "error: keycode 0x{v:x} has no scancode in layout {}",
                        layout.id
                    );
                    return ExitCode::from(1);
                }
            },
            None => {
                eprintln!("error: bad keycode value {value:?}");
                return ExitCode::from(1);
            }
        },
        "name" => {
            let Some(kc) = keycode_from_name(value) else {
                eprintln!("error: unknown SDL key name {value:?}");
                return ExitCode::from(1);
            };
            let Some(sc) = scancode_for(kc, layout.id) else {
                eprintln!(
                    "error: keycode for {value:?} (0x{:x}) has no scancode in layout {}",
                    kc.raw(),
                    layout.id
                );
                return ExitCode::from(1);
            };
            sc
        }
        other => {
            eprintln!("error: kind must be one of: scancode | keycode | name (got {other:?})");
            return ExitCode::from(1);
        }
    };

    let loc = MultiLocalizer::new();

    println!("Layout    : {} ({:?})", layout.id, layout.platform);
    println!("Locale    : {locale}");
    println!("Platform  : {platform:?}");
    println!("Style     : Textual + Symbolic shown side-by-side");
    println!(
        "Modifiers : {} (raw 0x{:04x})",
        format_mods(mods),
        mods.raw()
    );
    println!();

    println!(
        "Input scancode = {} (0x{:x})",
        scancode.raw(),
        scancode.raw()
    );

    // Forward resolve under both styles so the caller sees both.
    for style in [LabelStyle::Textual, LabelStyle::Symbolic] {
        let r = resolve(scancode, mods, layout.id, locale, style, &loc);
        println!();
        println!("── Resolved [{style:?}] ──");
        println!(
            "  scancode     : {} (0x{:x})",
            r.scancode.raw(),
            r.scancode.raw()
        );
        println!("  keycode      : 0x{:08x}", r.keycode.raw());
        println!("  character    : {:?}", r.character);
        println!("  named_key    : {:?}", r.named_key);
        println!("  glyph_en     : {:?}", r.glyph_en);
        println!("  glyph_local  : {:?}", r.glyph_local);
    }

    // Show the four modifier levels for this key (printable keys only).
    if let Some(key) = layout.key(scancode) {
        if key.named.is_none() {
            println!();
            println!("── Layout glyphs at every modifier level ──");
            println!("  base         : {:?}", key.base);
            println!("  shift        : {:?}", key.shift);
            println!("  altgr        : {:?}", key.altgr);
            println!("  shift+altgr  : {:?}", key.shift_altgr);
        }
    }

    // Modifier labels for every Modifier on the chosen platform/locale.
    println!();
    println!("── modifier_label() on {platform:?} for locale {locale:?} ──");
    for m in [
        Modifier::Ctrl,
        Modifier::Shift,
        Modifier::Alt,
        Modifier::Gui,
        Modifier::AltGr,
    ] {
        let t = modifier_label(m, platform, locale, LabelStyle::Textual, &loc);
        let s = modifier_label(m, platform, locale, LabelStyle::Symbolic, &loc);
        println!("  {m:<6?}  Textual={t:<12} Symbolic={s}");
    }

    // Cross-layout bridge: same physical key on every other layout.
    println!();
    println!(
        "── Same physical key (scancode {}) replayed on every layout ──",
        scancode.raw()
    );
    for other in all_layouts() {
        if other.id == layout.id {
            continue;
        }
        let r = resolve(scancode, mods, other.id, locale, LabelStyle::Textual, &loc);
        println!(
            "  {:<35}  char={:<5} named={:?}",
            other.id,
            format!("{:?}", r.character),
            r.named_key
        );
    }

    ExitCode::SUCCESS
}

fn usage() {
    eprintln!("usage: inspect <layout> <kind> <value> [mods] [locale] [platform]");
    eprintln!();
    eprintln!("  <layout>   layout id, or `?` to list every shipped layout");
    eprintln!("  <kind>     one of: scancode | keycode | name");
    eprintln!("  <value>    integer (dec or 0x…) for scancode/keycode, SDL name for `name`");
    eprintln!("  [mods]     comma-separated subset of: shift, ctrl, alt, gui, altgr, caps, num");
    eprintln!("  [locale]   BCP 47 tag (default: en)");
    eprintln!("  [platform] mac | windows | linux | chromeos | android (default: linux)");
}

fn parse_int(s: &str) -> Option<u32> {
    if let Some(rest) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(rest, 16).ok()
    } else {
        s.parse::<u32>().ok()
    }
}

fn parse_mods(s: &str) -> Result<KeyMod, String> {
    let mut m = KeyMod::NONE;
    if s.is_empty() {
        return Ok(m);
    }
    for tok in s.split(',') {
        let tok = tok.trim().to_ascii_lowercase();
        m |= match tok.as_str() {
            "" => continue,
            "shift" | "lshift" => KeyMod::LSHIFT,
            "rshift" => KeyMod::RSHIFT,
            "ctrl" | "lctrl" | "control" => KeyMod::LCTRL,
            "rctrl" => KeyMod::RCTRL,
            "alt" | "lalt" | "option" => KeyMod::LALT,
            "ralt" => KeyMod::RALT,
            "gui" | "lgui" | "cmd" | "command" | "win" | "windows" | "super" => KeyMod::LGUI,
            "rgui" => KeyMod::RGUI,
            "altgr" | "mode" => KeyMod::MODE,
            "caps" | "capslock" => KeyMod::CAPS,
            "num" | "numlock" => KeyMod::NUM,
            other => return Err(format!("unknown modifier {other:?}")),
        };
    }
    Ok(m)
}

fn format_mods(m: KeyMod) -> String {
    if m.is_empty() {
        return "none".to_string();
    }
    let mut parts = Vec::new();
    for (mask, name) in [
        (KeyMod::LSHIFT, "LSHIFT"),
        (KeyMod::RSHIFT, "RSHIFT"),
        (KeyMod::LCTRL, "LCTRL"),
        (KeyMod::RCTRL, "RCTRL"),
        (KeyMod::LALT, "LALT"),
        (KeyMod::RALT, "RALT"),
        (KeyMod::LGUI, "LGUI"),
        (KeyMod::RGUI, "RGUI"),
        (KeyMod::MODE, "MODE"),
        (KeyMod::CAPS, "CAPS"),
        (KeyMod::NUM, "NUM"),
    ] {
        if m.contains(mask) {
            parts.push(name);
        }
    }
    parts.join("+")
}
