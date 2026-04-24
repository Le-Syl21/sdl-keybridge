//! Logical keycodes (layout-aware) — SDL's `SDLK_*` constants.
//!
//! For printable keys the keycode is the Unicode code point of the base
//! character produced by the key in the current layout (e.g. `SDLK_A = 'a'`).
//! Non-printable keys are encoded as `scancode | SCANCODE_MASK`.

use crate::scancode::Scancode;

/// Bit OR-ed into non-printable keycodes to distinguish them from Unicode
/// code points. Value is identical in SDL2 and SDL3.
pub const SCANCODE_MASK: u32 = 1 << 30;

/// A logical keycode — either a Unicode code point or `scancode | SCANCODE_MASK`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct Keycode(pub u32);

impl Keycode {
    #[inline]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Create a keycode from a scancode (encodes with `SCANCODE_MASK`).
    #[inline]
    pub const fn from_scancode(sc: Scancode) -> Self {
        Self(sc.raw() | SCANCODE_MASK)
    }

    /// If this keycode is scancode-encoded, return the scancode portion.
    #[inline]
    pub const fn to_scancode(self) -> Option<Scancode> {
        if self.0 & SCANCODE_MASK != 0 {
            Some(Scancode::new(self.0 & !SCANCODE_MASK))
        } else {
            None
        }
    }

    /// If this keycode is a printable Unicode code point, return it as a char.
    #[inline]
    pub fn to_char(self) -> Option<char> {
        if self.0 & SCANCODE_MASK != 0 {
            None
        } else {
            char::from_u32(self.0)
        }
    }
}

impl From<u32> for Keycode {
    #[inline]
    fn from(value: u32) -> Self {
        Keycode(value)
    }
}

impl From<Keycode> for u32 {
    #[inline]
    fn from(value: Keycode) -> Self {
        value.0
    }
}

impl From<char> for Keycode {
    #[inline]
    fn from(c: char) -> Self {
        Keycode(c as u32)
    }
}

/// Common `SDLK_*` constants. Values identical in SDL2 and SDL3.
impl Keycode {
    pub const UNKNOWN: Keycode = Keycode(0);
    pub const RETURN: Keycode = Keycode(b'\r' as u32);
    pub const ESCAPE: Keycode = Keycode::from_scancode(Scancode::ESCAPE);
    pub const BACKSPACE: Keycode = Keycode(0x08);
    pub const TAB: Keycode = Keycode(b'\t' as u32);
    pub const SPACE: Keycode = Keycode(b' ' as u32);

    pub const EXCLAIM: Keycode = Keycode(b'!' as u32);
    pub const QUOTEDBL: Keycode = Keycode(b'"' as u32);
    pub const HASH: Keycode = Keycode(b'#' as u32);
    pub const DOLLAR: Keycode = Keycode(b'$' as u32);
    pub const PERCENT: Keycode = Keycode(b'%' as u32);
    pub const AMPERSAND: Keycode = Keycode(b'&' as u32);
    pub const QUOTE: Keycode = Keycode(b'\'' as u32);
    pub const LEFTPAREN: Keycode = Keycode(b'(' as u32);
    pub const RIGHTPAREN: Keycode = Keycode(b')' as u32);
    pub const ASTERISK: Keycode = Keycode(b'*' as u32);
    pub const PLUS: Keycode = Keycode(b'+' as u32);
    pub const COMMA: Keycode = Keycode(b',' as u32);
    pub const MINUS: Keycode = Keycode(b'-' as u32);
    pub const PERIOD: Keycode = Keycode(b'.' as u32);
    pub const SLASH: Keycode = Keycode(b'/' as u32);

    pub const NUM_0: Keycode = Keycode(b'0' as u32);
    pub const NUM_1: Keycode = Keycode(b'1' as u32);
    pub const NUM_2: Keycode = Keycode(b'2' as u32);
    pub const NUM_3: Keycode = Keycode(b'3' as u32);
    pub const NUM_4: Keycode = Keycode(b'4' as u32);
    pub const NUM_5: Keycode = Keycode(b'5' as u32);
    pub const NUM_6: Keycode = Keycode(b'6' as u32);
    pub const NUM_7: Keycode = Keycode(b'7' as u32);
    pub const NUM_8: Keycode = Keycode(b'8' as u32);
    pub const NUM_9: Keycode = Keycode(b'9' as u32);

    pub const COLON: Keycode = Keycode(b':' as u32);
    pub const SEMICOLON: Keycode = Keycode(b';' as u32);
    pub const LESS: Keycode = Keycode(b'<' as u32);
    pub const EQUALS: Keycode = Keycode(b'=' as u32);
    pub const GREATER: Keycode = Keycode(b'>' as u32);
    pub const QUESTION: Keycode = Keycode(b'?' as u32);
    pub const AT: Keycode = Keycode(b'@' as u32);
    pub const LEFTBRACKET: Keycode = Keycode(b'[' as u32);
    pub const BACKSLASH: Keycode = Keycode(b'\\' as u32);
    pub const RIGHTBRACKET: Keycode = Keycode(b']' as u32);
    pub const CARET: Keycode = Keycode(b'^' as u32);
    pub const UNDERSCORE: Keycode = Keycode(b'_' as u32);
    pub const BACKQUOTE: Keycode = Keycode(b'`' as u32);

    // Letters — lowercase Unicode code points.
    pub const A: Keycode = Keycode(b'a' as u32);
    pub const B: Keycode = Keycode(b'b' as u32);
    pub const C: Keycode = Keycode(b'c' as u32);
    pub const D: Keycode = Keycode(b'd' as u32);
    pub const E: Keycode = Keycode(b'e' as u32);
    pub const F: Keycode = Keycode(b'f' as u32);
    pub const G: Keycode = Keycode(b'g' as u32);
    pub const H: Keycode = Keycode(b'h' as u32);
    pub const I: Keycode = Keycode(b'i' as u32);
    pub const J: Keycode = Keycode(b'j' as u32);
    pub const K: Keycode = Keycode(b'k' as u32);
    pub const L: Keycode = Keycode(b'l' as u32);
    pub const M: Keycode = Keycode(b'm' as u32);
    pub const N: Keycode = Keycode(b'n' as u32);
    pub const O: Keycode = Keycode(b'o' as u32);
    pub const P: Keycode = Keycode(b'p' as u32);
    pub const Q: Keycode = Keycode(b'q' as u32);
    pub const R: Keycode = Keycode(b'r' as u32);
    pub const S: Keycode = Keycode(b's' as u32);
    pub const T: Keycode = Keycode(b't' as u32);
    pub const U: Keycode = Keycode(b'u' as u32);
    pub const V: Keycode = Keycode(b'v' as u32);
    pub const W: Keycode = Keycode(b'w' as u32);
    pub const X: Keycode = Keycode(b'x' as u32);
    pub const Y: Keycode = Keycode(b'y' as u32);
    pub const Z: Keycode = Keycode(b'z' as u32);

    // Non-printable — encoded via SCANCODE_MASK.
    pub const CAPSLOCK: Keycode = Keycode::from_scancode(Scancode::CAPSLOCK);

    pub const F1: Keycode = Keycode::from_scancode(Scancode::F1);
    pub const F2: Keycode = Keycode::from_scancode(Scancode::F2);
    pub const F3: Keycode = Keycode::from_scancode(Scancode::F3);
    pub const F4: Keycode = Keycode::from_scancode(Scancode::F4);
    pub const F5: Keycode = Keycode::from_scancode(Scancode::F5);
    pub const F6: Keycode = Keycode::from_scancode(Scancode::F6);
    pub const F7: Keycode = Keycode::from_scancode(Scancode::F7);
    pub const F8: Keycode = Keycode::from_scancode(Scancode::F8);
    pub const F9: Keycode = Keycode::from_scancode(Scancode::F9);
    pub const F10: Keycode = Keycode::from_scancode(Scancode::F10);
    pub const F11: Keycode = Keycode::from_scancode(Scancode::F11);
    pub const F12: Keycode = Keycode::from_scancode(Scancode::F12);
    pub const F13: Keycode = Keycode::from_scancode(Scancode::F13);
    pub const F14: Keycode = Keycode::from_scancode(Scancode::F14);
    pub const F15: Keycode = Keycode::from_scancode(Scancode::F15);
    pub const F16: Keycode = Keycode::from_scancode(Scancode::F16);
    pub const F17: Keycode = Keycode::from_scancode(Scancode::F17);
    pub const F18: Keycode = Keycode::from_scancode(Scancode::F18);
    pub const F19: Keycode = Keycode::from_scancode(Scancode::F19);
    pub const F20: Keycode = Keycode::from_scancode(Scancode::F20);
    pub const F21: Keycode = Keycode::from_scancode(Scancode::F21);
    pub const F22: Keycode = Keycode::from_scancode(Scancode::F22);
    pub const F23: Keycode = Keycode::from_scancode(Scancode::F23);
    pub const F24: Keycode = Keycode::from_scancode(Scancode::F24);

    pub const PRINT_SCREEN: Keycode = Keycode::from_scancode(Scancode::PRINT_SCREEN);
    pub const SCROLL_LOCK: Keycode = Keycode::from_scancode(Scancode::SCROLL_LOCK);
    pub const PAUSE: Keycode = Keycode::from_scancode(Scancode::PAUSE);
    pub const INSERT: Keycode = Keycode::from_scancode(Scancode::INSERT);
    pub const HOME: Keycode = Keycode::from_scancode(Scancode::HOME);
    pub const PAGE_UP: Keycode = Keycode::from_scancode(Scancode::PAGE_UP);
    pub const DELETE: Keycode = Keycode(0x7f);
    pub const END: Keycode = Keycode::from_scancode(Scancode::END);
    pub const PAGE_DOWN: Keycode = Keycode::from_scancode(Scancode::PAGE_DOWN);
    pub const RIGHT: Keycode = Keycode::from_scancode(Scancode::RIGHT);
    pub const LEFT: Keycode = Keycode::from_scancode(Scancode::LEFT);
    pub const DOWN: Keycode = Keycode::from_scancode(Scancode::DOWN);
    pub const UP: Keycode = Keycode::from_scancode(Scancode::UP);

    pub const NUM_LOCK_CLEAR: Keycode = Keycode::from_scancode(Scancode::NUM_LOCK_CLEAR);
    pub const KP_DIVIDE: Keycode = Keycode::from_scancode(Scancode::KP_DIVIDE);
    pub const KP_MULTIPLY: Keycode = Keycode::from_scancode(Scancode::KP_MULTIPLY);
    pub const KP_MINUS: Keycode = Keycode::from_scancode(Scancode::KP_MINUS);
    pub const KP_PLUS: Keycode = Keycode::from_scancode(Scancode::KP_PLUS);
    pub const KP_ENTER: Keycode = Keycode::from_scancode(Scancode::KP_ENTER);
    pub const KP_1: Keycode = Keycode::from_scancode(Scancode::KP_1);
    pub const KP_2: Keycode = Keycode::from_scancode(Scancode::KP_2);
    pub const KP_3: Keycode = Keycode::from_scancode(Scancode::KP_3);
    pub const KP_4: Keycode = Keycode::from_scancode(Scancode::KP_4);
    pub const KP_5: Keycode = Keycode::from_scancode(Scancode::KP_5);
    pub const KP_6: Keycode = Keycode::from_scancode(Scancode::KP_6);
    pub const KP_7: Keycode = Keycode::from_scancode(Scancode::KP_7);
    pub const KP_8: Keycode = Keycode::from_scancode(Scancode::KP_8);
    pub const KP_9: Keycode = Keycode::from_scancode(Scancode::KP_9);
    pub const KP_0: Keycode = Keycode::from_scancode(Scancode::KP_0);
    pub const KP_PERIOD: Keycode = Keycode::from_scancode(Scancode::KP_PERIOD);
    pub const KP_EQUALS: Keycode = Keycode::from_scancode(Scancode::KP_EQUALS);

    pub const APPLICATION: Keycode = Keycode::from_scancode(Scancode::APPLICATION);
    pub const MENU: Keycode = Keycode::from_scancode(Scancode::MENU);

    pub const LCTRL: Keycode = Keycode::from_scancode(Scancode::LCTRL);
    pub const LSHIFT: Keycode = Keycode::from_scancode(Scancode::LSHIFT);
    pub const LALT: Keycode = Keycode::from_scancode(Scancode::LALT);
    pub const LGUI: Keycode = Keycode::from_scancode(Scancode::LGUI);
    pub const RCTRL: Keycode = Keycode::from_scancode(Scancode::RCTRL);
    pub const RSHIFT: Keycode = Keycode::from_scancode(Scancode::RSHIFT);
    pub const RALT: Keycode = Keycode::from_scancode(Scancode::RALT);
    pub const RGUI: Keycode = Keycode::from_scancode(Scancode::RGUI);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printable_keycodes_are_unicode() {
        assert_eq!(Keycode::A.raw(), 0x61);
        assert_eq!(Keycode::Z.raw(), 0x7A);
        assert_eq!(Keycode::SPACE.raw(), 0x20);
        assert_eq!(Keycode::A.to_char(), Some('a'));
        assert_eq!(Keycode::SPACE.to_scancode(), None);
    }

    #[test]
    fn nonprintable_keycodes_use_scancode_mask() {
        assert!(Keycode::ESCAPE.raw() & SCANCODE_MASK != 0);
        assert_eq!(Keycode::ESCAPE.to_scancode(), Some(Scancode::ESCAPE));
        assert_eq!(Keycode::F5.to_scancode(), Some(Scancode::F5));
        assert_eq!(Keycode::F5.to_char(), None);
    }
}
