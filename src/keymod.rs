//! Modifier bitmask — SDL's `SDL_Keymod` / `KMOD_*` values.
//!
//! Values are shared across SDL2 and SDL3. SDL3 adds `LEVEL5 = 0x0004` which
//! did not exist in SDL2; callers targeting SDL2 should simply not set it.

/// Bitmask of modifier keys currently pressed / latched.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[repr(transparent)]
pub struct KeyMod(pub u16);

impl KeyMod {
    pub const NONE: KeyMod = KeyMod(0x0000);
    pub const LSHIFT: KeyMod = KeyMod(0x0001);
    pub const RSHIFT: KeyMod = KeyMod(0x0002);
    pub const LEVEL5: KeyMod = KeyMod(0x0004);
    pub const LCTRL: KeyMod = KeyMod(0x0040);
    pub const RCTRL: KeyMod = KeyMod(0x0080);
    pub const LALT: KeyMod = KeyMod(0x0100);
    pub const RALT: KeyMod = KeyMod(0x0200);
    pub const LGUI: KeyMod = KeyMod(0x0400);
    pub const RGUI: KeyMod = KeyMod(0x0800);
    pub const NUM: KeyMod = KeyMod(0x1000);
    pub const CAPS: KeyMod = KeyMod(0x2000);
    pub const MODE: KeyMod = KeyMod(0x4000);
    pub const SCROLL: KeyMod = KeyMod(0x8000);

    pub const SHIFT: KeyMod = KeyMod(Self::LSHIFT.0 | Self::RSHIFT.0);
    pub const CTRL: KeyMod = KeyMod(Self::LCTRL.0 | Self::RCTRL.0);
    pub const ALT: KeyMod = KeyMod(Self::LALT.0 | Self::RALT.0);
    pub const GUI: KeyMod = KeyMod(Self::LGUI.0 | Self::RGUI.0);

    #[inline]
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    #[inline]
    pub const fn raw(self) -> u16 {
        self.0
    }

    #[inline]
    pub const fn contains(self, other: KeyMod) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline]
    pub const fn intersects(self, other: KeyMod) -> bool {
        (self.0 & other.0) != 0
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// True if either Shift key is pressed.
    #[inline]
    pub const fn shift(self) -> bool {
        self.intersects(Self::SHIFT)
    }

    /// True if either Ctrl key is pressed.
    #[inline]
    pub const fn ctrl(self) -> bool {
        self.intersects(Self::CTRL)
    }

    /// True if either Alt key is pressed.
    #[inline]
    pub const fn alt(self) -> bool {
        self.intersects(Self::ALT)
    }

    /// True if either GUI (Win / Cmd / Super) key is pressed.
    #[inline]
    pub const fn gui(self) -> bool {
        self.intersects(Self::GUI)
    }

    /// True if the "AltGr" state is active — either `MODE` latched or `RALT`
    /// held (the usual encoding on Linux/Windows keyboards).
    #[inline]
    pub const fn altgr(self) -> bool {
        self.intersects(KeyMod(Self::MODE.0 | Self::RALT.0))
    }

    /// True if Caps Lock is currently latched.
    #[inline]
    pub const fn caps(self) -> bool {
        self.contains(Self::CAPS)
    }

    /// True if Num Lock is currently latched.
    #[inline]
    pub const fn num(self) -> bool {
        self.contains(Self::NUM)
    }
}

impl std::ops::BitOr for KeyMod {
    type Output = KeyMod;
    #[inline]
    fn bitor(self, rhs: KeyMod) -> KeyMod {
        KeyMod(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for KeyMod {
    type Output = KeyMod;
    #[inline]
    fn bitand(self, rhs: KeyMod) -> KeyMod {
        KeyMod(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for KeyMod {
    #[inline]
    fn bitor_assign(&mut self, rhs: KeyMod) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAndAssign for KeyMod {
    #[inline]
    fn bitand_assign(&mut self, rhs: KeyMod) {
        self.0 &= rhs.0;
    }
}

impl std::ops::Not for KeyMod {
    type Output = KeyMod;
    #[inline]
    fn not(self) -> KeyMod {
        KeyMod(!self.0)
    }
}

impl From<u16> for KeyMod {
    #[inline]
    fn from(value: u16) -> Self {
        KeyMod(value)
    }
}

impl From<KeyMod> for u16 {
    #[inline]
    fn from(value: KeyMod) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdl_compatible_values() {
        assert_eq!(KeyMod::LSHIFT.raw(), 0x0001);
        assert_eq!(KeyMod::LCTRL.raw(), 0x0040);
        assert_eq!(KeyMod::LALT.raw(), 0x0100);
        assert_eq!(KeyMod::LGUI.raw(), 0x0400);
        assert_eq!(KeyMod::NUM.raw(), 0x1000);
        assert_eq!(KeyMod::CAPS.raw(), 0x2000);
        assert_eq!(KeyMod::MODE.raw(), 0x4000);
    }

    #[test]
    fn modifier_groups() {
        assert!((KeyMod::LSHIFT | KeyMod::LCTRL).shift());
        assert!((KeyMod::LSHIFT | KeyMod::LCTRL).ctrl());
        assert!((KeyMod::RALT).altgr());
        assert!((KeyMod::MODE).altgr());
        assert!(!(KeyMod::LALT).altgr());
    }
}
