//! # sdl-keybridge — universal Rosetta Stone for SDL keyboards
//!
//! A static correspondence table that exposes, for every key press, all
//! of its parallel representations — physical [`Scancode`], logical
//! [`Keycode`], textual or symbolic glyph, localized label — across
//! every layout × platform combination supported by SDL.
//!
//! Works with both SDL2 and SDL3 (the crate takes primitive `u32` /
//! `u16` values whose numeric assignments are identical between the two
//! versions); no `sdl2` or `sdl3` Cargo feature is needed.
//!
//! ## Philosophy: Rosette, not Champollion
//!
//! This crate is a static lookup table (the **Rosette**). It exposes
//! parallel data and individual labels. It **does not interpret** and
//! **does not package** presentation conventions.
//!
//! Anything that *interprets* — formatting a `Ctrl+Shift+A` combo,
//! serializing a binding to disk, rebasing a config from one layout to
//! another — is **Champollion territory**: the job of consuming
//! applications, which *use* the Rosette to accomplish their task.
//!
//! Concretely, the crate ships:
//! - the four lookup primitives (forward / reverse / modifier / name parser);
//! - one extension trait, [`KeyLocalizer`], for plugging in custom labels.
//!
//! It does *not* ship `format_combo()`, a binding serializer, or layout
//! auto-detection. The README documents the canonical patterns the
//! consumer assembles in 2–3 lines of glue code.
//!
//! ## The four public functions
//!
//! | Function | Purpose |
//! |----------|---------|
//! | [`resolve`][fn@resolve]   | Forward lookup — every parallel representation in one pass |
//! | [`scancode_for`]          | Reverse lookup — physical key for a given keycode in a layout |
//! | [`modifier_label`]        | Platform-aware localized label for a held modifier |
//! | [`keycode_from_name`]     | Inverse of `SDL_GetKeyName` — parse a textual name |
//!
//! ## The one extension trait
//!
//! [`KeyLocalizer`] is the single point of customization. Implementors
//! map a stable id (`"key_escape"`, `"mod_gui_mac"`, …) to a display
//! string. The crate already ships a [`MultiLocalizer`] that aggregates
//! every locale module enabled through Cargo features — most callers
//! never need to write their own.
//!
//! Add your own only when you want to override or extend the default
//! labels (e.g. brand-specific naming):
//!
//! ```
//! use std::borrow::Cow;
//! use sdl_keybridge::{KeyLocalizer, LabelStyle};
//!
//! /// A localizer that overrides the macOS Command label with "⌘ Cmd"
//! /// and falls through to the built-in lookup for everything else.
//! struct BrandedLocalizer;
//!
//! impl KeyLocalizer for BrandedLocalizer {
//!     fn translate(&self, key_id: &str, _locale: &str, _style: LabelStyle)
//!         -> Option<Cow<'static, str>>
//!     {
//!         match key_id {
//!             "mod_gui_mac" => Some(Cow::Borrowed("⌘ Cmd")),
//!             _ => None, // fall back to the runtime lookup chain
//!         }
//!     }
//! }
//! ```
//!
//! ## Cross-layout binding bridge
//!
//! Two lines of consumer code translate a binding from one layout to
//! another through scancode-as-pivot:
//!
//! ```no_run
//! use sdl_keybridge::{resolve, scancode_for, Keycode, KeyMod, LabelStyle, MultiLocalizer};
//! # let keycode_ru = Keycode::from('ф');
//! let loc = MultiLocalizer::new();
//! let sc = scancode_for(keycode_ru, "windows/ru-t-k0-jcuken").unwrap();
//! let r  = resolve(sc, KeyMod::NONE, "windows/fr-t-k0-azerty", "fr", LabelStyle::Textual, &loc);
//! // r.glyph_local is what the user sees on their French AZERTY keyboard.
//! ```
//!
//! ## Module layout
//!
//! - [`scancode`] — [`Scancode`] newtype + USB HID / SDL constants
//! - [`keycode`]  — [`Keycode`] newtype + `SDLK_*` constants + scancode-mask helpers
//! - [`keymod`]   — [`KeyMod`] modifier bitmask (`KMOD_*`) + ergonomic predicates
//! - [`named_key`] — [`NamedKey`] taxonomy with stable `"key_*"` ids
//! - [`layout`]   — [`Layout`] / [`LayoutKey`] structs + the static layout registry
//! - [`localizer`] — [`KeyLocalizer`] trait, [`MultiLocalizer`], [`Platform`], [`Modifier`], [`LabelStyle`]
//! - [`mod@resolve`]  — the four public functions
//!
//! ## Cargo features
//!
//! Locales are feature-gated. The default feature is `en`. Pull only
//! what you ship for, or use the `all-locales` aggregate.
//!
//! ```toml
//! [dependencies]
//! sdl-keybridge = { version = "0.1", features = ["fr", "de", "ja"] }
//! ```
//!
//! Available codes: `ar`, `bn`, `cs`, `de`, `en`, `es`, `fi`, `fr`,
//! `hi`, `id`, `it`, `ja`, `ko`, `nl`, `pl`, `pt`, `ru`, `sk`, `sv`,
//! `sw`, `th`, `tr`, `ur`, `vi`, `zh-hans`, `zh-hant`.
//!
//! ## Bundled examples
//!
//! Two `examples/` binaries ship with the crate to validate the API
//! without writing your own driver:
//!
//! - `cargo run --example showcase --all-features` — non-interactive
//!   walk-through of the eleven canonical scenarios (cross-layout char
//!   resolution, locale fallback, Caps/NumLock correctness, platform
//!   modifier glyphs, the bridge pattern, combo formatting).
//! - `cargo run --example inspect --all-features -- <layout> <kind> <value> [mods] [locale] [platform]`
//!   — interactive inspector. Feed it a layout id and either a
//!   `scancode N`, a `keycode N` (decimal or hex), or a `name "Esc"`
//!   and it dumps the full [`Resolved`] in both styles, the four-level
//!   layout glyph table, every [`modifier_label`] for the chosen
//!   platform, and replays the same physical key on every other layout
//!   (cross-layout bridge demo). Pass `?` as the layout id to list the
//!   layouts compiled into the build.
//!
//! ## Non-goals
//!
//! Permanent — these will not land in this crate:
//!
//! - **Detecting the current OS layout.** No Rust solution reliably
//!   covers all five SDL platforms; the caller supplies the BCP 47 id.
//! - **Dead keys / text composition.** A scancode + modifiers resolves
//!   to *one* glyph. Composition (`^` + `e` → `ê`) is the OS/IME's job
//!   via `SDL_StartTextInput`, not ours.
//! - **`format_combo()`.** Presentation conventions vary; the consumer
//!   assembles labels with its preferred separator and ordering.
//! - **Binding serialization.** Config format is the consumer's choice
//!   (JSON, INI, RON, binary, …).
//! - **`sdl2` / `sdl3` feature flags.** A single API on primitive types
//!   covers both versions.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod keycode;
pub mod keymod;
pub mod layout;
pub mod localizer;
pub mod named_key;
pub mod resolve;
pub mod scancode;

mod locales;

pub use keycode::{Keycode, SCANCODE_MASK};
pub use keymod::KeyMod;
pub use layout::{all_layouts, get_layout, Layout, LayoutKey, LAYOUTS, STD_NAMED_KEYS};
pub use localizer::{KeyLocalizer, LabelStyle, Modifier, MultiLocalizer, Platform};
pub use named_key::NamedKey;
pub use resolve::{keycode_from_name, modifier_label, resolve, scancode_for, Resolved};
pub use scancode::Scancode;
