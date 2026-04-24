//! Build-time generator: parses the CLDR 43 LDML-KEYBOARD 2.x XML files
//! shipped under `data/cldr-43/keyboards/` and emits a single Rust source
//! file (`$OUT_DIR/cldr_layouts.rs`) included by `src/layout.rs`.
//!
//! Output:
//! - `CLDR_KEYS_<SYMBOL>: &[LayoutKey]` — one const per layout.
//! - `CLDR_LAYOUT_<SYMBOL>: Layout`    — one const per layout.
//! - `CLDR_LAYOUTS: &[&Layout]`         — the flat registry.
//!
//! Layouts whose `<keyMap>` blocks only use exotic modifier combinations
//! (cmd, fn, ctrl-without-alt) are skipped — the 4-level model of this
//! crate only covers Base / Shift / AltGr / Shift+AltGr.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use quick_xml::events::Event;
use quick_xml::reader::Reader;

const ISO_MAP: &[(&str, &str)] = &[
    ("E00", "GRAVE"),
    ("E01", "NUM_1"),
    ("E02", "NUM_2"),
    ("E03", "NUM_3"),
    ("E04", "NUM_4"),
    ("E05", "NUM_5"),
    ("E06", "NUM_6"),
    ("E07", "NUM_7"),
    ("E08", "NUM_8"),
    ("E09", "NUM_9"),
    ("E10", "NUM_0"),
    ("E11", "MINUS"),
    ("E12", "EQUALS"),
    ("E13", "INTERNATIONAL3"),
    ("D01", "Q"),
    ("D02", "W"),
    ("D03", "E"),
    ("D04", "R"),
    ("D05", "T"),
    ("D06", "Y"),
    ("D07", "U"),
    ("D08", "I"),
    ("D09", "O"),
    ("D10", "P"),
    ("D11", "LEFT_BRACKET"),
    ("D12", "RIGHT_BRACKET"),
    ("D13", "BACKSLASH"),
    ("C01", "A"),
    ("C02", "S"),
    ("C03", "D"),
    ("C04", "F"),
    ("C05", "G"),
    ("C06", "H"),
    ("C07", "J"),
    ("C08", "K"),
    ("C09", "L"),
    ("C10", "SEMICOLON"),
    ("C11", "APOSTROPHE"),
    ("C12", "NON_US_HASH"),
    ("B00", "NON_US_BACKSLASH"),
    ("B01", "Z"),
    ("B02", "X"),
    ("B03", "C"),
    ("B04", "V"),
    ("B05", "B"),
    ("B06", "N"),
    ("B07", "M"),
    ("B08", "COMMA"),
    ("B09", "PERIOD"),
    ("B10", "SLASH"),
    ("B11", "INTERNATIONAL1"),
    ("B12", "INTERNATIONAL2"),
];

fn iso_to_scancode_name(iso: &str) -> Option<&'static str> {
    ISO_MAP
        .iter()
        .find_map(|&(k, v)| if k == iso { Some(v) } else { None })
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Level {
    Base,
    Shift,
    AltGr,
    ShiftAltGr,
}

/// Classify a `<keyMap modifiers="...">` attribute into one of our four
/// levels, or return `None` for exotic combinations (cmd, fn,
/// ctrl-without-alt) that do not map cleanly.
fn classify(mods: Option<&str>) -> Option<Level> {
    let Some(raw) = mods else {
        return Some(Level::Base);
    };
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Some(Level::Base);
    }
    // CLDR uses `shift?` for "optional shift" and `caps?` for "optional
    // caps"; treat optional markers as presence for this crude classifier.
    let cleaned = trimmed.replace('?', "");
    // Space-separated groups are disjunctions — take the primary group.
    let group = cleaned.split_whitespace().next().unwrap_or("");
    let tokens: Vec<&str> = group
        .split('+')
        .filter(|t| !t.is_empty() && *t != "caps")
        .collect();
    let has_shift = tokens.contains(&"shift");
    let has_altgr = tokens.contains(&"altR")
        || tokens.contains(&"opt")
        || (tokens.contains(&"ctrl") && tokens.contains(&"alt"));
    let has_cmd_or_fn = tokens.iter().any(|t| matches!(*t, "cmd" | "fn"));
    let ctrl_alone = tokens.contains(&"ctrl") && !tokens.contains(&"alt");
    if has_cmd_or_fn || ctrl_alone {
        return None;
    }
    Some(match (has_shift, has_altgr) {
        (false, false) => Level::Base,
        (true, false) => Level::Shift,
        (false, true) => Level::AltGr,
        (true, true) => Level::ShiftAltGr,
    })
}

/// Decode the `to=...` value into a single `char`. Returns `None` if
/// the value decodes to zero or more than one Unicode scalar (we do not
/// represent ligatures or multi-character outputs in the layout model).
fn decode_to(raw: &str) -> Option<char> {
    let decoded = decode_unicode_escapes(raw);
    let mut it = decoded.chars();
    let c = it.next()?;
    if it.next().is_some() {
        return None;
    }
    if c.is_control() && c != ' ' {
        return None;
    }
    Some(c)
}

/// Rewrite CLDR's `\u{HHHH}` escapes into their decoded characters.
/// XML entities (`&amp;`, `&apos;`, etc.) are already unescaped by the
/// XML parser upstream.
fn decode_unicode_escapes(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        if i + 3 < bytes.len() && bytes[i] == b'\\' && bytes[i + 1] == b'u' && bytes[i + 2] == b'{'
        {
            if let Some(close_off) = bytes[i + 3..].iter().position(|&b| b == b'}') {
                let hex = &s[i + 3..i + 3 + close_off];
                if let Ok(cp) = u32::from_str_radix(hex, 16) {
                    if let Some(c) = char::from_u32(cp) {
                        out.push(c);
                        i += 3 + close_off + 1;
                        continue;
                    }
                }
            }
        }
        // Safe: we always advance by a whole UTF-8 char.
        let ch = s[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

#[derive(Debug)]
struct LayoutData {
    id: String,
    display_name: String,
    platform_const: &'static str,
    language: String,
    // scancode_name -> [base, shift, altgr, shift_altgr]
    levels: BTreeMap<&'static str, [Option<char>; 4]>,
}

fn platform_for(dir_name: &str, filename_stem: &str) -> (&'static str, &'static str) {
    match dir_name {
        "android" => ("android", "Platform::Android"),
        "chromeos" => ("chromeos", "Platform::ChromeOS"),
        "osx" => ("mac", "Platform::Mac"),
        "windows" => ("windows", "Platform::Windows"),
        "und" => {
            if filename_stem.contains("-windows") {
                ("windows", "Platform::Windows")
            } else if filename_stem.contains("-osx") {
                ("mac", "Platform::Mac")
            } else if filename_stem.contains("-android") {
                ("android", "Platform::Android")
            } else if filename_stem.contains("-chromeos") {
                ("chromeos", "Platform::ChromeOS")
            } else {
                ("linux", "Platform::Linux")
            }
        }
        _ => ("linux", "Platform::Linux"),
    }
}

fn parse_file(path: &Path, platform_dir: &str) -> Option<LayoutData> {
    let content = fs::read_to_string(path).ok()?;
    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut locale = String::new();
    let mut display_name = String::new();
    let mut current_level: Option<Level> = None;
    let mut in_keymap = false;
    let mut levels: BTreeMap<&'static str, [Option<char>; 4]> = BTreeMap::new();

    loop {
        let event = reader.read_event_into(&mut buf);
        match event {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => match e.name().as_ref() {
                b"keyboard" => {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"locale" {
                            if let Ok(v) = attr.unescape_value() {
                                locale = v.into_owned();
                            }
                        }
                    }
                }
                b"name" => {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"value" {
                            if let Ok(v) = attr.unescape_value() {
                                display_name = v.into_owned();
                            }
                        }
                    }
                }
                b"keyMap" => {
                    in_keymap = true;
                    let mut modifiers: Option<String> = None;
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"modifiers" {
                            if let Ok(v) = attr.unescape_value() {
                                modifiers = Some(v.into_owned());
                            }
                        }
                    }
                    current_level = classify(modifiers.as_deref());
                }
                b"map" if in_keymap => {
                    let Some(level) = current_level else {
                        continue;
                    };
                    let mut iso: Option<String> = None;
                    let mut to: Option<String> = None;
                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"iso" => {
                                if let Ok(v) = attr.unescape_value() {
                                    iso = Some(v.into_owned());
                                }
                            }
                            b"to" => {
                                if let Ok(v) = attr.unescape_value() {
                                    to = Some(v.into_owned());
                                }
                            }
                            _ => {}
                        }
                    }
                    let (Some(iso), Some(to)) = (iso, to) else {
                        continue;
                    };
                    let Some(sc_name) = iso_to_scancode_name(&iso) else {
                        continue;
                    };
                    let Some(c) = decode_to(&to) else {
                        continue;
                    };
                    let entry = levels.entry(sc_name).or_insert([None; 4]);
                    let idx = match level {
                        Level::Base => 0,
                        Level::Shift => 1,
                        Level::AltGr => 2,
                        Level::ShiftAltGr => 3,
                    };
                    if entry[idx].is_none() {
                        entry[idx] = Some(c);
                    }
                }
                _ => {}
            },
            Ok(Event::End(ref e)) if e.name().as_ref() == b"keyMap" => {
                in_keymap = false;
                current_level = None;
            }
            Ok(Event::Eof) => break,
            Err(err) => {
                println!(
                    "cargo:warning=failed to parse {:?}: {err}",
                    path.file_name().unwrap_or_default()
                );
                return None;
            }
            _ => {}
        }
        buf.clear();
    }

    if locale.is_empty() {
        return None;
    }
    let stem = path.file_stem()?.to_str()?;
    let (prefix, platform_const) = platform_for(platform_dir, stem);
    let id = format!("{}/{}", prefix, locale);
    let language = locale.split('-').next().unwrap_or(&locale).to_string();
    if display_name.is_empty() {
        display_name = format!("{locale} (CLDR)");
    }

    Some(LayoutData {
        id,
        display_name,
        platform_const,
        language,
        levels,
    })
}

fn sanitize_ident(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_uppercase());
        } else {
            out.push('_');
        }
    }
    out
}

fn rs_char(c: char) -> String {
    match c {
        '\\' => String::from("'\\\\'"),
        '\'' => String::from("'\\''"),
        '\0' => String::from("'\\0'"),
        c if (' '..='~').contains(&c) => format!("'{c}'"),
        c => format!("'\\u{{{:X}}}'", c as u32),
    }
}

fn rs_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => out.push_str(&format!("\\u{{{:X}}}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn main() {
    println!("cargo:rerun-if-changed=data/cldr-43/keyboards");
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let base = manifest_dir.join("data/cldr-43/keyboards");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let out_path = out_dir.join("cldr_layouts.rs");

    let mut layouts: Vec<LayoutData> = Vec::new();
    if base.exists() {
        for platform_dir in ["android", "chromeos", "osx", "windows", "und"] {
            let dir = base.join(platform_dir);
            if !dir.exists() {
                continue;
            }
            let entries = match fs::read_dir(&dir) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) != Some("xml") {
                    continue;
                }
                if let Some(l) = parse_file(&path, platform_dir) {
                    if !l.levels.is_empty() {
                        layouts.push(l);
                    }
                }
            }
        }
    }

    layouts.sort_by(|a, b| a.id.cmp(&b.id));
    layouts.dedup_by(|a, b| a.id == b.id);

    let mut out = String::new();
    out.push_str("// @generated by build.rs from data/cldr-43/keyboards — do not edit\n\n");

    for l in &layouts {
        let sym = sanitize_ident(&l.id);
        out.push_str(&format!("const CLDR_KEYS_{sym}: &[LayoutKey] = &[\n"));
        for (sc_name, glyphs) in &l.levels {
            let Some(base_c) = glyphs[0] else {
                continue;
            };
            let shift = glyphs[1];
            let altgr = glyphs[2];
            let shift_altgr = glyphs[3];
            if altgr.is_some() || shift_altgr.is_some() {
                out.push_str(&format!(
                    "    LayoutKey::printable4(Scancode::{}, {}, {}, {}, {}),\n",
                    sc_name,
                    rs_char(base_c),
                    rs_char(shift.unwrap_or('\0')),
                    rs_char(altgr.unwrap_or('\0')),
                    rs_char(shift_altgr.unwrap_or('\0')),
                ));
            } else {
                let shift_c = shift.unwrap_or(base_c);
                out.push_str(&format!(
                    "    LayoutKey::printable(Scancode::{}, {}, {}),\n",
                    sc_name,
                    rs_char(base_c),
                    rs_char(shift_c),
                ));
            }
        }
        out.push_str("];\n\n");

        out.push_str(&format!(
            "pub const CLDR_LAYOUT_{sym}: Layout = Layout {{\n    \
             id: {},\n    \
             display_name: {},\n    \
             platform: {},\n    \
             language: {},\n    \
             named_keys: STD_NAMED_KEYS,\n    \
             printable_keys: CLDR_KEYS_{sym},\n}};\n\n",
            rs_str(&l.id),
            rs_str(&l.display_name),
            l.platform_const,
            rs_str(&l.language),
        ));
    }

    out.push_str("pub const CLDR_LAYOUTS: &[&Layout] = &[\n");
    for l in &layouts {
        let sym = sanitize_ident(&l.id);
        out.push_str(&format!("    &CLDR_LAYOUT_{sym},\n"));
    }
    out.push_str("];\n");

    fs::write(&out_path, out).expect("write cldr_layouts.rs");
    println!(
        "cargo:warning=sdl-keybridge: generated {} CLDR layouts",
        layouts.len()
    );
}
