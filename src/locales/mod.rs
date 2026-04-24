//! Locale modules. Each `<code>.rs` is feature-gated and exposes a single
//! `translate(key_id, style) -> Option<Cow<'static, str>>` function.
//!
//! Adding a new locale:
//! 1. `cp src/locales/en.rs src/locales/<code>.rs`
//! 2. Translate the strings in the `match`.
//! 3. Add `<code> = []` in `Cargo.toml` under `[features]` and to the
//!    `all-locales` feature aggregate.
//! 4. Add the `#[cfg(feature = "<code>")]` entries below in this file.

use std::borrow::Cow;

use crate::localizer::LabelStyle;

#[cfg(feature = "ar")]
pub(crate) mod ar;
#[cfg(feature = "bn")]
pub(crate) mod bn;
#[cfg(feature = "cs")]
pub(crate) mod cs;
#[cfg(feature = "de")]
pub(crate) mod de;
#[cfg(feature = "en")]
pub(crate) mod en;
#[cfg(feature = "es")]
pub(crate) mod es;
#[cfg(feature = "fi")]
pub(crate) mod fi;
#[cfg(feature = "fr")]
pub(crate) mod fr;
#[cfg(feature = "hi")]
pub(crate) mod hi;
#[cfg(feature = "id")]
pub(crate) mod id;
#[cfg(feature = "it")]
pub(crate) mod it;
#[cfg(feature = "ja")]
pub(crate) mod ja;
#[cfg(feature = "ko")]
pub(crate) mod ko;
#[cfg(feature = "nl")]
pub(crate) mod nl;
#[cfg(feature = "pl")]
pub(crate) mod pl;
#[cfg(feature = "pt")]
pub(crate) mod pt;
#[cfg(feature = "ru")]
pub(crate) mod ru;
#[cfg(feature = "sk")]
pub(crate) mod sk;
#[cfg(feature = "sv")]
pub(crate) mod sv;
#[cfg(feature = "sw")]
pub(crate) mod sw;
#[cfg(feature = "th")]
pub(crate) mod th;
#[cfg(feature = "tr")]
pub(crate) mod tr;
#[cfg(feature = "ur")]
pub(crate) mod ur;
#[cfg(feature = "vi")]
pub(crate) mod vi;
#[cfg(feature = "zh-hans")]
#[path = "zh-hans.rs"]
pub(crate) mod zh_hans;
#[cfg(feature = "zh-hant")]
#[path = "zh-hant.rs"]
pub(crate) mod zh_hant;

/// Look up a translation by locale code. Returns `None` if the locale
/// is not compiled in or does not cover the key id.
pub(crate) fn translate(
    locale: &str,
    key_id: &str,
    style: LabelStyle,
) -> Option<Cow<'static, str>> {
    // Normalize a few BCP 47 aliases — we accept the base tag only.
    let locale_norm = normalize_locale(locale);
    match locale_norm {
        #[cfg(feature = "ar")]
        "ar" => ar::translate(key_id, style),
        #[cfg(feature = "bn")]
        "bn" => bn::translate(key_id, style),
        #[cfg(feature = "cs")]
        "cs" => cs::translate(key_id, style),
        #[cfg(feature = "de")]
        "de" => de::translate(key_id, style),
        #[cfg(feature = "en")]
        "en" => en::translate(key_id, style),
        #[cfg(feature = "es")]
        "es" => es::translate(key_id, style),
        #[cfg(feature = "fi")]
        "fi" => fi::translate(key_id, style),
        #[cfg(feature = "fr")]
        "fr" => fr::translate(key_id, style),
        #[cfg(feature = "hi")]
        "hi" => hi::translate(key_id, style),
        #[cfg(feature = "id")]
        "id" => id::translate(key_id, style),
        #[cfg(feature = "it")]
        "it" => it::translate(key_id, style),
        #[cfg(feature = "ja")]
        "ja" => ja::translate(key_id, style),
        #[cfg(feature = "ko")]
        "ko" => ko::translate(key_id, style),
        #[cfg(feature = "nl")]
        "nl" => nl::translate(key_id, style),
        #[cfg(feature = "pl")]
        "pl" => pl::translate(key_id, style),
        #[cfg(feature = "pt")]
        "pt" => pt::translate(key_id, style),
        #[cfg(feature = "ru")]
        "ru" => ru::translate(key_id, style),
        #[cfg(feature = "sk")]
        "sk" => sk::translate(key_id, style),
        #[cfg(feature = "sv")]
        "sv" => sv::translate(key_id, style),
        #[cfg(feature = "sw")]
        "sw" => sw::translate(key_id, style),
        #[cfg(feature = "th")]
        "th" => th::translate(key_id, style),
        #[cfg(feature = "tr")]
        "tr" => tr::translate(key_id, style),
        #[cfg(feature = "ur")]
        "ur" => ur::translate(key_id, style),
        #[cfg(feature = "vi")]
        "vi" => vi::translate(key_id, style),
        #[cfg(feature = "zh-hans")]
        "zh-hans" => zh_hans::translate(key_id, style),
        #[cfg(feature = "zh-hant")]
        "zh-hant" => zh_hant::translate(key_id, style),
        _ => None,
    }
}

/// Map a few well-known BCP 47 aliases down to our feature codes.
///
/// - `"en-US"`, `"en-GB"` → `"en"`
/// - `"fr-FR"`, `"fr-CA"` → `"fr"`
/// - `"zh-CN"`, `"zh-SG"`, `"zh-Hans-*"` → `"zh-hans"`
/// - `"zh-TW"`, `"zh-HK"`, `"zh-Hant-*"` → `"zh-hant"`
fn normalize_locale(locale: &str) -> &str {
    let lower = locale;
    // Chinese script disambiguation.
    let lower_alloc = lower.to_ascii_lowercase();
    if lower_alloc.starts_with("zh") {
        if lower_alloc.contains("hant")
            || lower_alloc.ends_with("-tw")
            || lower_alloc.ends_with("-hk")
            || lower_alloc.ends_with("-mo")
        {
            return "zh-hant";
        }
        if lower_alloc.contains("hans")
            || lower_alloc.ends_with("-cn")
            || lower_alloc.ends_with("-sg")
            || lower_alloc == "zh"
        {
            return "zh-hans";
        }
    }
    // Otherwise strip the region subtag.
    match lower.split_once('-') {
        Some((base, _)) => base_to_static(base),
        None => base_to_static(lower),
    }
}

/// Map the base-tag slice back onto a `&'static str` that matches our
/// feature codes. Returns `""` when unknown.
fn base_to_static(base: &str) -> &'static str {
    match base.to_ascii_lowercase().as_str() {
        "ar" => "ar",
        "bn" => "bn",
        "cs" => "cs",
        "de" => "de",
        "en" => "en",
        "es" => "es",
        "fi" => "fi",
        "fr" => "fr",
        "hi" => "hi",
        "id" => "id",
        "it" => "it",
        "ja" => "ja",
        "ko" => "ko",
        "nl" => "nl",
        "pl" => "pl",
        "pt" => "pt",
        "ru" => "ru",
        "sk" => "sk",
        "sv" => "sv",
        "sw" => "sw",
        "th" => "th",
        "tr" => "tr",
        "ur" => "ur",
        "vi" => "vi",
        _ => "",
    }
}
