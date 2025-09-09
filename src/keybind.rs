use x11rb::protocol::xproto::KeyButMask;

pub enum KeybindAction {
    Resize(u32, u32),
    Quit,
    Close,
}

#[repr(u8)]
pub enum Keycode {
    Q = 24,
    W = 25,
    E = 26,
    R = 27,
    // TODO: etc.
}

pub struct Keybind {
    pub mask: KeyButMask,
    pub key: u8,
    pub command: KeybindAction,
}

impl Keybind {
    pub fn new(mask: KeyButMask, key: Keycode, command: KeybindAction) -> Self {
        Self {
            mask: mask,
            key: key as u8,
            command: command,
        }
    }
}
