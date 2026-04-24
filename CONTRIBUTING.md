# Contributing to sdl-keybridge

Thanks for your interest! The crate is dual-licensed **MIT OR Apache-2.0**.
By submitting a pull request you agree your contribution is licensed
under those same terms.

## Before pushing

Every push must be fmt-clean and clippy-clean:

```sh
cargo fmt --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
```

For the minimum feature matrix CI:

```sh
cargo test --no-default-features --features en
cargo test --no-default-features --features fr
cargo test --all-features
cargo test
```

## Adding or completing a translation

**We actively need native-speaker contributions** for the partial
locales listed in `README.md` — especially **Arabic, Bengali, Hindi,
Indonesian, Swahili, Thai, Urdu, Vietnamese**. Their current state
only translates the 20-odd most common keys; anything else (F-keys,
keypad labels, left/right modifier variants) falls back to English.
The maintainers deliberately did *not* auto-translate those to avoid
introducing subtle errors in scripts we cannot verify.

Adding a **brand-new locale** (Greek, Hebrew, Icelandic, Kazakh,
Ukrainian, …) is equally welcome — ~120 lines of translated strings
plus ~5 lines of wiring.

The locale modules are **pure Rust** — no TOML, no code generation, no
macros. Each is one `match` expression.

1. Copy the reference file:
   ```sh
   cp src/locales/en.rs src/locales/<your-code>.rs
   ```
2. Translate the strings. You need to cover:
   - Every `key_id` returned by `NamedKey::key_id()` — see the list in
     `src/named_key.rs`.
   - Every `mod_*` id emitted by `modifier_label()` — see the arms in
     `src/locales/en.rs`.
3. Leave any id you are *not* sure about returning `None`. The runtime
   fallback chain (locale → en) will fill the gap.
4. Add the feature flag in `Cargo.toml`:
   ```toml
   [features]
   <your-code> = []
   all-locales = [..., "<your-code>"]
   ```
5. Wire it in `src/locales/mod.rs`:
   - Add the `#[cfg(feature = "<your-code>")] pub(crate) mod <your_code>;`
     declaration at the top.
   - Add the `#[cfg(feature = "<your-code>")] "<your-code>" => <your_code>::translate(...)`
     arm inside `translate()`.
   - If the tag has an ASCII-only base form (e.g. `zh-hans`), add the
     base mapping in `base_to_static()`. For script-hinted codes use a
     module path attribute: `#[path = "zh-hans.rs"]`.
6. Run the tests:
   ```sh
   cargo test --features <your-code>
   cargo test --all-features
   ```

### Translation guidelines

- **Symbolic style** glyphs (`⇧`, `⌘`, `↑↓←→`, …) are Unicode and
  essentially culture-neutral. If your locale does not change the glyph
  (which is the common case), return `None` for `Symbolic` and let the
  fallback chain serve the English glyph. See `src/locales/fi.rs` for
  an example of a Textual-only locale.
- **Modifier keys** — macOS spells out `Command` / `Option` / `Control`
  in full; elsewhere the convention is the short form `Ctrl` / `Alt` /
  `Windows` / `Super`. The `mod_<name>_<platform>` ids encode this.
- **Left/right variants** (`key_shift_left`, …) — not every locale
  cares. Leave them `None` if your language does not have an idiomatic
  translation.

## Adding a layout

v0.1 layouts are hand-curated in `src/layout.rs`. Each layout is a
`Layout` struct referencing:

- A shared `STD_NAMED_KEYS` constant (escape, arrows, F-keys, keypad,
  modifiers). Do not duplicate this — every layout reuses it.
- A per-layout `PRINTABLE_KEYS` constant defining the character-
  producing keys for every scancode position, at up to four modifier
  levels (Base / Shift / AltGr / Shift+AltGr).

To add a layout:

1. Define the `PRINTABLE_KEYS` constant next to the existing ones
   (`QWERTY_US_KEYS`, `AZERTY_FR_KEYS`, …).
2. Define one or more `LAYOUT_<PLATFORM>_<ID>` constants referencing it.
3. Append them to `LAYOUTS` at the bottom of the file.
4. Add at least one dedicated round-trip assertion in
   `tests/layouts.rs`.

### Roadmap: CLDR-generated layouts

Long-term, every layout under `keyboards/3.0/` in [unicode-org/cldr][cldr]
should be importable through a `build.rs` that parses the LDML XML and
emits the static tables. This was deliberately deferred from v0.1 to
keep the initial crate reviewable. Contributions welcome.

[cldr]: https://github.com/unicode-org/cldr/tree/main/keyboards

## Reporting bugs / suggesting features

Open an issue on GitHub with:

- the layout id and locale you used,
- the scancode / keycode / modifier values (and SDL version, if
  relevant),
- what you got vs. what you expected.

## Code of conduct

Be kind. Be precise about the problem. Be specific about the fix.
