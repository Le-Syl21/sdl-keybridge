//! Localization trait and UI-style enums.
//!
//! The core crate ships the [`KeyLocalizer`] trait and a [`MultiLocalizer`]
//! that aggregates the locale modules enabled via Cargo features. Each
//! locale module implements the trait and is wired in through the
//! internal `locales` module.

use std::borrow::Cow;

use crate::keymod::KeyMod;

/// How a key should be rendered for display.
///
/// - `Textual`  — words like `"Up"`, `"Haut"`, `"Command"`.
/// - `Symbolic` — single-glyph icons like `↑`, `⌘`, `⇧`.
///
/// Consumers pick the style that fits their UI.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum LabelStyle {
    #[default]
    Textual,
    Symbolic,
}

/// Host operating system — required to pick the correct glyph and label
/// for modifier keys (e.g. `LGUI` → `⌘` on macOS, `⊞` on Windows,
/// `◇` / `Super` on Linux).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Platform {
    Mac,
    Windows,
    Linux,
    ChromeOS,
    Android,
}

impl Platform {
    /// Short id suffix used in modifier `key_id`s — e.g.
    /// `"mod_gui_mac"` maps to `Platform::Mac`.
    pub const fn id(self) -> &'static str {
        match self {
            Platform::Mac => "mac",
            Platform::Windows => "win",
            Platform::Linux => "linux",
            Platform::ChromeOS => "chromeos",
            Platform::Android => "android",
        }
    }
}

/// Logical modifier — platform-agnostic. The mapping to physical
/// `LCTRL` / `RCTRL` / `LGUI` / `LALT` / `MODE` / `LEVEL5` bits is
/// done inside [`resolve`](crate::resolve()) and
/// [`modifier_label`](crate::modifier_label).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Modifier {
    Ctrl,
    Shift,
    Alt,
    Gui,
    AltGr,
}

impl Modifier {
    /// Stable id prefix used in localizer lookups — e.g. `"mod_ctrl"`.
    pub const fn key_id_prefix(self) -> &'static str {
        match self {
            Modifier::Ctrl => "mod_ctrl",
            Modifier::Shift => "mod_shift",
            Modifier::Alt => "mod_alt",
            Modifier::Gui => "mod_gui",
            Modifier::AltGr => "mod_altgr",
        }
    }

    /// The `KeyMod` bitmask that represents *either side* of this modifier.
    pub const fn mask(self) -> KeyMod {
        match self {
            Modifier::Ctrl => KeyMod::CTRL,
            Modifier::Shift => KeyMod::SHIFT,
            Modifier::Alt => KeyMod::ALT,
            Modifier::Gui => KeyMod::GUI,
            Modifier::AltGr => KeyMod::new(KeyMod::MODE.raw() | KeyMod::RALT.raw()),
        }
    }
}

/// The crate's single extension point — translate stable ids
/// (`"key_escape"`, `"mod_gui_mac"`, …) into display strings.
///
/// # Stable id contract
///
/// The id passed to [`translate`](Self::translate) is one of:
///
/// - `NamedKey::key_id()` — for non-printable keys, see
///   [`crate::NamedKey`] for the full list (`"key_escape"`,
///   `"key_arrow_up"`, `"key_kp_7"`, …).
/// - `"mod_<name>_<platform>"` — for held-modifier labels emitted by
///   [`crate::modifier_label`]: `"mod_ctrl"` / `"mod_shift"` /
///   `"mod_alt"` / `"mod_gui"` / `"mod_altgr"`, optionally suffixed
///   with `_mac` / `_win` / `_linux` / `_chromeos` / `_android`.
///
/// New ids may be added in future versions; existing ids will never be
/// renamed.
///
/// # Style awareness
///
/// Translations may differ based on [`LabelStyle`] — e.g. `Symbolic`
/// returns `↑` while `Textual` returns `"Haut"` (French). When your
/// translation is the same for both styles, return it for both.
///
/// # Fallback chain
///
/// A localizer answering for only a subset of ids (or only one locale)
/// must return `None` for everything else. The crate's runtime walks
/// this chain on every public-API call:
///
/// 1. Your custom localizer for `(key_id, locale, style)`.
/// 2. The compiled-in locale module for `locale`.
/// 3. The English (`"en"`) module.
/// 4. The raw id as a last resort.
///
/// # Implementing a custom localizer
///
/// Most callers use [`MultiLocalizer`] which already aggregates every
/// locale shipped via Cargo features. Implement your own only to
/// override or extend specific labels.
///
/// ```
/// use std::borrow::Cow;
/// use sdl_keybridge::{KeyLocalizer, LabelStyle, MultiLocalizer};
///
/// /// Renames every macOS GUI label to "⌘ Cmd" for our brand UI.
/// /// Falls through (returns `None`) for everything else so the runtime
/// /// chain still serves the rest from MultiLocalizer.
/// struct BrandedLocalizer;
///
/// impl KeyLocalizer for BrandedLocalizer {
///     fn translate(
///         &self,
///         key_id: &str,
///         _locale: &str,
///         _style: LabelStyle,
///     ) -> Option<Cow<'static, str>> {
///         match key_id {
///             "mod_gui_mac" => Some(Cow::Borrowed("⌘ Cmd")),
///             _ => None,
///         }
///     }
/// }
///
/// // Pass it where you'd normally pass MultiLocalizer.
/// let _ = BrandedLocalizer;
/// let _fallback = MultiLocalizer::new();
/// ```
///
/// # Implementing a single-locale localizer
///
/// To add a brand-new locale (rather than override one), follow the
/// `CONTRIBUTING.md` recipe: copy `src/locales/en.rs` and wire it up
/// behind a Cargo feature. That gets the locale into [`MultiLocalizer`]
/// for free, which is almost always what users want.
///
/// You can also implement [`KeyLocalizer`] yourself for an unsupported
/// locale — return `Some(_)` only when `locale` matches your code:
///
/// ```
/// use std::borrow::Cow;
/// use sdl_keybridge::{KeyLocalizer, LabelStyle};
///
/// struct EsperantoLocalizer;
///
/// impl KeyLocalizer for EsperantoLocalizer {
///     fn translate(&self, key_id: &str, locale: &str, style: LabelStyle)
///         -> Option<Cow<'static, str>>
///     {
///         if locale != "eo" { return None; }
///         use LabelStyle::*;
///         let s = match (key_id, style) {
///             ("key_escape", Textual)    => "Eskapi",
///             ("key_return", Textual)    => "Enen",
///             ("key_space",  Textual)    => "Spaco",
///             _ => return None,
///         };
///         Some(Cow::Borrowed(s))
///     }
/// }
/// ```
pub trait KeyLocalizer {
    /// Translate a stable id into a display string, or `None` if this
    /// localizer has no opinion on `(key_id, locale, style)`.
    ///
    /// Returning `None` lets the runtime fallback chain take over —
    /// see the trait-level docs for the chain order.
    fn translate(&self, key_id: &str, locale: &str, style: LabelStyle)
        -> Option<Cow<'static, str>>;
}

/// Aggregates all locale modules compiled into the crate and dispatches
/// calls by locale code. Use this as the default `KeyLocalizer` when you
/// don't need custom overrides.
#[derive(Copy, Clone, Debug, Default)]
pub struct MultiLocalizer {
    _private: (),
}

impl MultiLocalizer {
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl KeyLocalizer for MultiLocalizer {
    fn translate(
        &self,
        key_id: &str,
        locale: &str,
        style: LabelStyle,
    ) -> Option<Cow<'static, str>> {
        crate::locales::translate(locale, key_id, style)
    }
}

/// Helper used by the public API: try the user-supplied localizer first,
/// then fall back to the compiled locale set, finally to English.
pub(crate) fn translate_for(
    key_id: &str,
    locale: &str,
    style: LabelStyle,
    localizer: &impl KeyLocalizer,
) -> Option<Cow<'static, str>> {
    if let Some(s) = localizer.translate(key_id, locale, style) {
        return Some(s);
    }
    if let Some(s) = crate::locales::translate(locale, key_id, style) {
        return Some(s);
    }
    crate::locales::translate("en", key_id, style)
}
