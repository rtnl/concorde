use x11rb::protocol::xproto::KeyButMask;

pub enum KeybindAction {
    SwitchToWorkspace(u8),
    MoveToWorkspace(u8, u8),
    Focus(u8),
    Resize(u32, u32),
    Fullscreen(u8),
    Execute(String),
    Close,
    Quit,
}

#[repr(u8)]
pub enum Keycode {
    Esc = 9,
    Q = 24,
    W = 25,
    E = 26,
    R = 27,
    // TODO: etc.
}

pub struct Keybind {
    pub mask: KeyButMask,
    pub key: u8,
    pub action: KeybindAction,
}

impl Keybind {
    pub fn new(mask: KeyButMask, key: Keycode, action: KeybindAction) -> Self {
        Self {
            mask: mask,
            key: key as u8,
            action: action,
        }
    }
}
