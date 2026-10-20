use rmk::types::action::KeyAction;
use rmk::{a, k, kbctrl, layer, mo, user};

pub const COLS: usize = 15;
pub const ROWS: usize = 5;
pub const LAYERS: usize = 3;

// Fn layer: the stock HHKB in Mac mode (NumLock is keypad Clear), plus reboot
// on R, bootloader on U and the Bluetooth layer on Control.
// User keys 0 to 2 select Bluetooth profiles (hold 5 s to forget the bond),
// 3 and 4 are next and previous profile, 5 clears the current bond and 6
// toggles between USB and Bluetooth.
#[rustfmt::skip]
pub const fn default_keymap() -> [[[KeyAction; COLS]; ROWS]; LAYERS] {
    [
        layer!([
            [k!(Escape), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), k!(Minus), k!(Equal), k!(Backslash), k!(Grave)],
            [k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Y), k!(U), k!(I), k!(O), k!(P), k!(LeftBracket), k!(RightBracket), k!(Backspace), a!(No)],
            [k!(LCtrl), k!(A), k!(S), k!(D), k!(F), k!(G), k!(H), k!(J), k!(K), k!(L), k!(Semicolon), k!(Quote), k!(Enter), a!(No), a!(No)],
            [k!(LShift), k!(Z), k!(X), k!(C), k!(V), k!(B), k!(N), k!(M), k!(Comma), k!(Dot), k!(Slash), k!(RShift), mo!(1), a!(No), a!(No)],
            [k!(LAlt), k!(LGui), k!(Space), k!(RGui), k!(RAlt), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [k!(KbPower), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6), k!(F7), k!(F8), k!(F9), k!(F10), k!(F11), k!(F12), k!(Insert), k!(Delete)],
            [k!(CapsLock), a!(Transparent), a!(Transparent), a!(Transparent), kbctrl!(Reboot), a!(Transparent), a!(Transparent), kbctrl!(Bootloader), k!(PrintScreen), k!(ScrollLock), k!(Pause), k!(Up), a!(Transparent), k!(NumLock), a!(No)],
            [mo!(2), k!(AudioVolDown), k!(AudioVolUp), k!(AudioMute), k!(MediaEject), a!(Transparent), k!(KpAsterisk), k!(KpSlash), k!(Home), k!(PageUp), k!(Left), k!(Right), k!(KpEnter), a!(No), a!(No)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), k!(KpPlus), k!(KpMinus), k!(End), k!(PageDown), k!(Down), a!(Transparent), a!(Transparent), a!(No), a!(No)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [user!(6), user!(0), user!(1), user!(2), user!(3), user!(4), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), user!(5), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
    ]
}
