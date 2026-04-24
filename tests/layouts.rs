//! Per-layout round-trip and coverage tests.

use sdl_keybridge::{all_layouts, resolve, scancode_for, KeyMod, LabelStyle, MultiLocalizer};

#[test]
fn every_printable_key_round_trips_in_its_layout() {
    // Some real-world layouts have two physical keys producing the same
    // base character (e.g. `/` on both BACKSLASH and NON_US_BACKSLASH in
    // some Belgian layouts). In that case `scancode_for` returns *a*
    // scancode whose base keycode matches — not necessarily the one we
    // started from. The invariant we verify here is the weaker but
    // still meaningful round-trip:
    //
    //   resolve(scancode_for(kc)).keycode == kc
    let loc = MultiLocalizer::new();

    for layout in all_layouts().iter() {
        for k in layout.printable_keys.iter() {
            let r = resolve(
                k.scancode,
                KeyMod::NONE,
                layout.id,
                "en",
                LabelStyle::Textual,
                &loc,
            );
            let kc = r.keycode;
            let sc = scancode_for(kc, layout.id).unwrap_or_else(|| {
                panic!("no reverse lookup for {:?} in layout {}", kc, layout.id)
            });
            let round = resolve(sc, KeyMod::NONE, layout.id, "en", LabelStyle::Textual, &loc);
            assert_eq!(
                round.keycode, kc,
                "round-trip keycode mismatch for layout={} scancode={:?} -> keycode={:?} -> scancode={:?} -> keycode={:?}",
                layout.id, k.scancode, kc, sc, round.keycode
            );
        }
    }
}

#[test]
fn azerty_fr_swap_preserves_russian_f_to_french_q() {
    // A Russian user creates a binding on the letter "Ф" (scancode A on
    // JCUKEN). Bridge that to a French AZERTY keyboard.
    let loc = MultiLocalizer::new();

    let keycode_ru_f = sdl_keybridge::Keycode::from('ф');
    let sc = scancode_for(keycode_ru_f, "windows/ru-t-k0-windows")
        .expect("ф must exist in the Russian layout");
    assert_eq!(sc, sdl_keybridge::Scancode::A);

    let r = resolve(
        sc,
        KeyMod::NONE,
        "windows/fr-t-k0-windows",
        "fr",
        LabelStyle::Textual,
        &loc,
    );
    // On AZERTY, the `A` scancode position is labeled `q`.
    assert_eq!(r.character, Some('q'));
}

#[test]
fn german_y_swap() {
    let loc = MultiLocalizer::new();

    // On QWERTY, scancode Y gives 'y'; on QWERTZ, it gives 'z'.
    let r_qwerty = resolve(
        sdl_keybridge::Scancode::Y,
        KeyMod::NONE,
        "windows/en-t-k0-windows",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r_qwerty.character, Some('y'));

    let r_qwertz = resolve(
        sdl_keybridge::Scancode::Y,
        KeyMod::NONE,
        "windows/de-t-k0-windows",
        "de",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r_qwertz.character, Some('z'));
}

#[test]
fn dvorak_a_is_still_a() {
    let loc = MultiLocalizer::new();
    let r = resolve(
        sdl_keybridge::Scancode::A,
        KeyMod::NONE,
        "windows/en-t-k0-windows-dvorak",
        "en",
        LabelStyle::Textual,
        &loc,
    );
    assert_eq!(r.character, Some('a'));
}
