# sdl-keybridge

[![CI](https://github.com/Le-Syl21/sdl-keybridge/actions/workflows/ci.yml/badge.svg)](https://github.com/Le-Syl21/sdl-keybridge/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/sdl-keybridge.svg)](https://crates.io/crates/sdl-keybridge)
[![Docs.rs](https://docs.rs/sdl-keybridge/badge.svg)](https://docs.rs/sdl-keybridge)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

**The universal Rosetta Stone for SDL keyboards.**

A static correspondence table that exposes, for every key press, all of
its parallel representations — physical scancode, logical keycode,
textual or symbolic glyph, localized label — across all the layout ×
platform combinations supported by SDL.

No other Rust crate combines **layout-awareness** (AZERTY/QWERTZ/JCUKEN),
**i18n of named keys** (Escape → Échap/Esc/エスケープ), and a
**cross-layout binding bridge** via scancode as a universal pivot.

## Philosophy: Rosette, not Champollion

This crate is a **static lookup table** (Rosette). It exposes parallel
data and individual labels. It **does not interpret** and **does not
package** presentation conventions.

Anything that *interprets* — formatting a `Ctrl+Shift+A` combo,
serializing a binding, rebasing a config from one layout to another — is
**Champollion territory**: the job of the consuming application. The
Rosette gives you the raw correspondence data; you pick the separator,
the ordering, the storage format.

## The four public functions

```rust
use sdl_keybridge::{
    resolve, scancode_for, modifier_label, keycode_from_name,
    KeyMod, Keycode, LabelStyle, Modifier, MultiLocalizer, Platform, Scancode,
};

let loc = MultiLocalizer::new();

// 1. Forward lookup — every parallel representation in one pass.
let r = resolve(
    Scancode::A,
    KeyMod::LSHIFT,
    "linux/fr-t-k0-azerty",  // layout id
    "fr",                     // UI locale
    LabelStyle::Textual,
    &loc,
);
assert_eq!(r.character, Some('Q'));  // Shift+A on AZERTY → 'Q'

// 2. Reverse lookup — find the scancode for a keycode in a layout.
let sc = scancode_for(Keycode::from('ф'), "linux/ru-t-k0-jcuken");
assert_eq!(sc, Some(Scancode::A));

// 3. Platform-aware modifier label.
let s = modifier_label(Modifier::Gui, Platform::Mac, "fr", LabelStyle::Textual, &loc);
assert_eq!(s.as_ref(), "Commande");

// 4. Parse a textual key name back into a keycode.
assert_eq!(keycode_from_name("Left Shift"), Some(Keycode::LSHIFT));
```

## Cross-layout binding bridge

Two lines of consumer code translate a binding from one layout to
another through scancode-as-pivot:

```rust
use sdl_keybridge::{resolve, scancode_for, KeyMod, Keycode, LabelStyle, MultiLocalizer};

# let keycode_ru = Keycode::from('ф');
let loc = MultiLocalizer::new();
let sc = scancode_for(keycode_ru, "windows/ru-t-k0-jcuken").unwrap();
let r = resolve(sc, KeyMod::NONE, "windows/fr-t-k0-azerty", "fr", LabelStyle::Textual, &loc);
// r.glyph_local is what the user sees on their French AZERTY keyboard.
```

## Combo formatting (consumer-side)

There is no `format_combo()` in this crate — the separator and ordering
are yours. Assemble the individual labels the way your UI expects:

```rust
use sdl_keybridge::{resolve, modifier_label, KeyMod, LabelStyle, MultiLocalizer, Modifier, Platform, Scancode};

# let loc = MultiLocalizer::new();
# let layout = "linux/fr-t-k0-azerty";
# let locale = "fr";
# let style = LabelStyle::Textual;
let ctrl = modifier_label(Modifier::Ctrl, Platform::Linux, locale, style, &loc);
let r = resolve(Scancode::A, KeyMod::LCTRL, layout, locale, style, &loc);
let combo = format!("{}+{}", ctrl, r.glyph_local);  // "Ctrl+q" on AZERTY
```

## SDL2 + SDL3 compatibility

There are no `sdl2` / `sdl3` feature flags. The API takes primitive types
(`u32` for scancodes / keycodes, `u16` for the modifier bitmask), whose
numeric values are identical between SDL2 and SDL3 for every constant
exposed here. Use the same crate regardless of your SDL binding.

## Locales

26 locales available as individual Cargo features:

| Code | Language | Code | Language | Code | Language |
| --- | --- | --- | --- | --- | --- |
| `en` (default) | English | `fr` | Français | `de` | Deutsch |
| `es` | Español | `it` | Italiano | `pt` | Português |
| `nl` | Nederlands | `sv` | Svenska | `fi` | Suomi |
| `pl` | Polski | `cs` | Čeština | `sk` | Slovenčina |
| `tr` | Türkçe | `ru` | Русский | `ar` | العربية |
| `hi` | हिन्दी | `bn` | বাংলা | `ur` | اردو |
| `zh-hans` | 简体中文 | `zh-hant` | 繁體中文 | `ja` | 日本語 |
| `ko` | 한국어 | `id` | Bahasa Indonesia | `sw` | Kiswahili |
| `vi` | Tiếng Việt | `th` | ภาษาไทย | | |

Enable only what you need; use the aggregate `all-locales` feature to
pull them all in.

```toml
[dependencies]
sdl-keybridge = { version = "0.1", features = ["fr", "de", "ja"] }
```

## Layouts

v0.3 ships **601 layouts, all sourced from [Unicode CLDR 43](https://github.com/unicode-org/cldr/tree/release-43/keyboards)**
at build time — no hand-authored layout data in the crate:

| Platform | Count |
| --- | ---: |
| Android  | 175 |
| ChromeOS |  55 |
| macOS    | 137 |
| Windows  | 209 |
| Platform-neutral (`und`) | 25 |
| **Total** | **601** |

Every glyph served by `resolve()` / `scancode_for()` comes from a
CLDR-maintained XML with the full chain of Unicode Consortium / vendor
review behind it. If you find a character that looks wrong, that's a
CLDR upstream issue — fix it there and refresh `data/cldr-43/` to get
the corrected version.

Layout ids are the exact CLDR locale tags prefixed by the platform dir:

- `windows/en-t-k0-windows`           — US QWERTY, Windows
- `windows/en-GB-t-k0-windows`        — UK QWERTY
- `windows/fr-t-k0-windows`           — French AZERTY
- `windows/de-t-k0-windows`           — German QWERTZ
- `windows/ru-t-k0-windows`           — Russian ЙЦУКЕН
- `windows/ja-t-k0-windows`           — Japanese JIS
- `osx/en-t-k0-osx-colemak`           — Colemak (macOS)
- `windows/en-t-k0-windows-dvorak`    — Dvorak (Windows)
- `android/*-t-k0-android*`           — touch keyboards with long-press
                                        ignored (we expose primary glyph)
- etc.

CLDR 44+ migrated to a new 3.0 format currently covering only ~8 niche
layouts; CLDR 43 remains the canonical legacy source until 3.0 catches
up.

Run `cargo run --example inspect --all-features -- ? scancode 0` for
the full list of layout ids in this build.

### Regenerating from fresh CLDR data

The build script reads `data/cldr-43/keyboards/**/*.xml`. To switch
source (e.g. drop in a newer CLDR release), replace that directory and
rebuild — the generated output regenerates automatically. Parser lives
in `build.rs`; ISO-to-scancode mapping is the `ISO_MAP` table at the
top of that file.

## What this crate will *not* do (non-goals)

- **Detect the current OS layout** — the caller provides the BCP 47 id.
  No Rust solution reliably covers all five SDL platforms.
- **Dead keys / text composition** — a scancode + modifiers resolves to
  *one* glyph. Composition (e.g. `^` + `e` → `ê`) is the OS/IME's job,
  triggered by `SDL_StartTextInput`, not by us.
- **Package a `format_combo()`** — presentation conventions vary; the
  consumer assembles the labels it receives.
- **Package a binding serializer** — config format is yours (JSON, INI,
  RON, binary, …).

## License

Dual-licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
