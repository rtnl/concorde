mod axis;
mod box2;
mod keybind;
pub mod vec2;

use std::process::{Command, exit};

use crate::axis::Axis;
use crate::box2::Box2f;
use crate::keybind::{Keybind, KeybindAction, Keycode};

use crate::vec2::Vec2f;

use x11rb::atom_manager;
use x11rb::errors::ReplyError;
use x11rb::protocol::xproto::KeyButMask;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt, EventMask, Screen},
    protocol::{ErrorKind, Event},
};

struct Concorde<'a, C> {
    connection: &'a C,
    screen: &'a Screen,
    layout: Box2f,
    keybinds: Vec<Keybind>,
}

atom_manager! {
    pub AtomCollection:

    AtomCollectionCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        WM_STATE,
        WM_TAKE_FOCUS,
        _NET_ACTIVE_WINDOW,
        _NET_SUPPORTED,
        _NET_WM_NAME,
        _NET_WM_STATE,
        _NET_SUPPORTING_WM_CHECK,
        _NET_WM_STATE_FULLSCREEN,
        _NET_WM_WINDOW_TYPE,
        _NET_WM_WINDOW_TYPE_DIALOG,
        _NET_CLIENT_LIST,
    }
}

impl<'a, C: Connection> Concorde<'a, C> {
    fn new(connection: &'a C, screen: &'a Screen, keybinds: Vec<Keybind>) -> Self {
        Self {
            connection: connection,
            screen: screen,
            layout: Box2f::new(
                Vec2f::zero(),
                Vec2f::new(
                    screen.width_in_pixels as f64,
                    screen.height_in_pixels as f64,
                ),
            ),
            keybinds: keybinds,
        }
    }

    fn manage_windows(&self) -> Result<(), ReplyError> {
        let change = ChangeWindowAttributesAux::default().event_mask(
            EventMask::SUBSTRUCTURE_REDIRECT
                | EventMask::SUBSTRUCTURE_NOTIFY
                | EventMask::KEY_RELEASE,
        );

        let result = self
            .connection
            .change_window_attributes(self.screen.root, &change)?
            .check();

        if let Err(ReplyError::X11Error(ref error)) = result {
            if error.error_kind == ErrorKind::Access {
                eprintln!("Another window manager is running.");
                exit(1);
            }
        }

        result
    }

    fn setup(&self) -> Result<(), Box<dyn std::error::Error>> {
        let atoms = AtomCollection::new(self.connection)?;
        Ok(())
    }

    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.connection.flush()?;

            let event = self.connection.wait_for_event()?;
            let mut event_option = Some(event);

            while let Some(ref event) = event_option {
                match event {
                    Event::KeyRelease(event) => {
                        for keybind in &self.keybinds {
                            if event.detail == keybind.key && event.state == keybind.mask {
                                match &keybind.action {
                                    KeybindAction::Quit => return Ok(()),
                                    KeybindAction::Execute(command) => {
                                        Command::new(command).spawn()?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }

                self.connection.poll_for_event()?;
                event_option = self.connection.poll_for_event()?;
            }
        }
    }
}

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
