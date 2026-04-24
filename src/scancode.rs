//! Physical key positions — SDL scancodes.
//!
//! A scancode identifies a physical key on the keyboard independently of the
//! current layout. Values are compatible with both SDL2 and SDL3 (identical
//! numeric assignments for the keys covered here).

/// A physical key position, wrapping the raw SDL scancode value.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct Scancode(pub u32);

impl Scancode {
    #[inline]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl From<u32> for Scancode {
    #[inline]
    fn from(value: u32) -> Self {
        Scancode(value)
    }
}

impl From<Scancode> for u32 {
    #[inline]
    fn from(value: Scancode) -> Self {
        value.0
    }
}

/// SDL scancode constants — stable across SDL2 and SDL3.
impl Scancode {
    pub const UNKNOWN: Scancode = Scancode(0);

    // Letters (USB HID keyboard page, SDL layout).
    pub const A: Scancode = Scancode(4);
    pub const B: Scancode = Scancode(5);
    pub const C: Scancode = Scancode(6);
    pub const D: Scancode = Scancode(7);
    pub const E: Scancode = Scancode(8);
    pub const F: Scancode = Scancode(9);
    pub const G: Scancode = Scancode(10);
    pub const H: Scancode = Scancode(11);
    pub const I: Scancode = Scancode(12);
    pub const J: Scancode = Scancode(13);
    pub const K: Scancode = Scancode(14);
    pub const L: Scancode = Scancode(15);
    pub const M: Scancode = Scancode(16);
    pub const N: Scancode = Scancode(17);
    pub const O: Scancode = Scancode(18);
    pub const P: Scancode = Scancode(19);
    pub const Q: Scancode = Scancode(20);
    pub const R: Scancode = Scancode(21);
    pub const S: Scancode = Scancode(22);
    pub const T: Scancode = Scancode(23);
    pub const U: Scancode = Scancode(24);
    pub const V: Scancode = Scancode(25);
    pub const W: Scancode = Scancode(26);
    pub const X: Scancode = Scancode(27);
    pub const Y: Scancode = Scancode(28);
    pub const Z: Scancode = Scancode(29);

    // Top-row digits (1..9 then 0 — SDL layout).
    pub const NUM_1: Scancode = Scancode(30);
    pub const NUM_2: Scancode = Scancode(31);
    pub const NUM_3: Scancode = Scancode(32);
    pub const NUM_4: Scancode = Scancode(33);
    pub const NUM_5: Scancode = Scancode(34);
    pub const NUM_6: Scancode = Scancode(35);
    pub const NUM_7: Scancode = Scancode(36);
    pub const NUM_8: Scancode = Scancode(37);
    pub const NUM_9: Scancode = Scancode(38);
    pub const NUM_0: Scancode = Scancode(39);

    pub const RETURN: Scancode = Scancode(40);
    pub const ESCAPE: Scancode = Scancode(41);
    pub const BACKSPACE: Scancode = Scancode(42);
    pub const TAB: Scancode = Scancode(43);
    pub const SPACE: Scancode = Scancode(44);

    pub const MINUS: Scancode = Scancode(45);
    pub const EQUALS: Scancode = Scancode(46);
    pub const LEFT_BRACKET: Scancode = Scancode(47);
    pub const RIGHT_BRACKET: Scancode = Scancode(48);
    pub const BACKSLASH: Scancode = Scancode(49);
    pub const NON_US_HASH: Scancode = Scancode(50);
    pub const SEMICOLON: Scancode = Scancode(51);
    pub const APOSTROPHE: Scancode = Scancode(52);
    pub const GRAVE: Scancode = Scancode(53);
    pub const COMMA: Scancode = Scancode(54);
    pub const PERIOD: Scancode = Scancode(55);
    pub const SLASH: Scancode = Scancode(56);

    pub const CAPSLOCK: Scancode = Scancode(57);

    // Function keys F1..F12.
    pub const F1: Scancode = Scancode(58);
    pub const F2: Scancode = Scancode(59);
    pub const F3: Scancode = Scancode(60);
    pub const F4: Scancode = Scancode(61);
    pub const F5: Scancode = Scancode(62);
    pub const F6: Scancode = Scancode(63);
    pub const F7: Scancode = Scancode(64);
    pub const F8: Scancode = Scancode(65);
    pub const F9: Scancode = Scancode(66);
    pub const F10: Scancode = Scancode(67);
    pub const F11: Scancode = Scancode(68);
    pub const F12: Scancode = Scancode(69);

    pub const PRINT_SCREEN: Scancode = Scancode(70);
    pub const SCROLL_LOCK: Scancode = Scancode(71);
    pub const PAUSE: Scancode = Scancode(72);
    pub const INSERT: Scancode = Scancode(73);
    pub const HOME: Scancode = Scancode(74);
    pub const PAGE_UP: Scancode = Scancode(75);
    pub const DELETE: Scancode = Scancode(76);
    pub const END: Scancode = Scancode(77);
    pub const PAGE_DOWN: Scancode = Scancode(78);
    pub const RIGHT: Scancode = Scancode(79);
    pub const LEFT: Scancode = Scancode(80);
    pub const DOWN: Scancode = Scancode(81);
    pub const UP: Scancode = Scancode(82);

    pub const NUM_LOCK_CLEAR: Scancode = Scancode(83);

    // Keypad.
    pub const KP_DIVIDE: Scancode = Scancode(84);
    pub const KP_MULTIPLY: Scancode = Scancode(85);
    pub const KP_MINUS: Scancode = Scancode(86);
    pub const KP_PLUS: Scancode = Scancode(87);
    pub const KP_ENTER: Scancode = Scancode(88);
    pub const KP_1: Scancode = Scancode(89);
    pub const KP_2: Scancode = Scancode(90);
    pub const KP_3: Scancode = Scancode(91);
    pub const KP_4: Scancode = Scancode(92);
    pub const KP_5: Scancode = Scancode(93);
    pub const KP_6: Scancode = Scancode(94);
    pub const KP_7: Scancode = Scancode(95);
    pub const KP_8: Scancode = Scancode(96);
    pub const KP_9: Scancode = Scancode(97);
    pub const KP_0: Scancode = Scancode(98);
    pub const KP_PERIOD: Scancode = Scancode(99);

    pub const NON_US_BACKSLASH: Scancode = Scancode(100);
    pub const APPLICATION: Scancode = Scancode(101);
    pub const POWER: Scancode = Scancode(102);
    pub const KP_EQUALS: Scancode = Scancode(103);

    pub const F13: Scancode = Scancode(104);
    pub const F14: Scancode = Scancode(105);
    pub const F15: Scancode = Scancode(106);
    pub const F16: Scancode = Scancode(107);
    pub const F17: Scancode = Scancode(108);
    pub const F18: Scancode = Scancode(109);
    pub const F19: Scancode = Scancode(110);
    pub const F20: Scancode = Scancode(111);
    pub const F21: Scancode = Scancode(112);
    pub const F22: Scancode = Scancode(113);
    pub const F23: Scancode = Scancode(114);
    pub const F24: Scancode = Scancode(115);

    pub const MENU: Scancode = Scancode(118);

    // International keys — JIS, Korean, Brazilian layouts.
    pub const INTERNATIONAL1: Scancode = Scancode(135);
    pub const INTERNATIONAL2: Scancode = Scancode(136);
    pub const INTERNATIONAL3: Scancode = Scancode(137); // yen
    pub const INTERNATIONAL4: Scancode = Scancode(138);
    pub const INTERNATIONAL5: Scancode = Scancode(139);
    pub const INTERNATIONAL6: Scancode = Scancode(140);
    pub const INTERNATIONAL7: Scancode = Scancode(141);
    pub const INTERNATIONAL8: Scancode = Scancode(142);
    pub const INTERNATIONAL9: Scancode = Scancode(143);
    pub const LANG1: Scancode = Scancode(144); // Hangul/English toggle
    pub const LANG2: Scancode = Scancode(145); // Hanja
    pub const LANG3: Scancode = Scancode(146); // Katakana
    pub const LANG4: Scancode = Scancode(147); // Hiragana
    pub const LANG5: Scancode = Scancode(148); // Zenkaku/Hankaku

    // Modifiers.
    pub const LCTRL: Scancode = Scancode(224);
    pub const LSHIFT: Scancode = Scancode(225);
    pub const LALT: Scancode = Scancode(226);
    pub const LGUI: Scancode = Scancode(227);
    pub const RCTRL: Scancode = Scancode(228);
    pub const RSHIFT: Scancode = Scancode(229);
    pub const RALT: Scancode = Scancode(230);
    pub const RGUI: Scancode = Scancode(231);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_scancode_values() {
        assert_eq!(Scancode::A.raw(), 4);
        assert_eq!(Scancode::ESCAPE.raw(), 41);
        assert_eq!(Scancode::SPACE.raw(), 44);
        assert_eq!(Scancode::NUM_1.raw(), 30);
        assert_eq!(Scancode::NUM_0.raw(), 39);
        assert_eq!(Scancode::LSHIFT.raw(), 225);
        assert_eq!(Scancode::RGUI.raw(), 231);
    }
}
