mod axis;
mod box2;
mod keybind;
mod node;
mod vec2;
mod wm;

use crate::keybind::{Keybind, KeybindAction, Keycode};
use crate::wm::Concorde;
use std::process::exit;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::KeyButMask;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Ok((connection, screen_number)) = x11rb::connect(None) else {
        eprintln!("Failed to connect to X server. (is it running?)");
        exit(1);
    };

    // TODO: parse .toml config
    let keybinds = vec![
        Keybind::new(
            KeyButMask::SHIFT | KeyButMask::MOD4,
            Keycode::Esc,
            KeybindAction::Quit,
        ),
        Keybind::new(
            KeyButMask::SHIFT | KeyButMask::MOD4,
            Keycode::R,
            KeybindAction::Execute("dmenu_run".to_string()),
        ),
    ];

    let concorde = Concorde::new(
        &connection,
        &connection.setup().roots[screen_number],
        keybinds,
    );

    concorde.manage_windows()?;
    concorde.setup()?;
    concorde.run()
}
