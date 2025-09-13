use crate::keybind::{Keybind, KeybindAction};
use crate::node::WindowNode;
use parking_lot::Mutex;
use std::process::{exit, Command};
use std::sync::Arc;
use x11rb::atom_manager;
use x11rb::connection::Connection;
use x11rb::errors::ReplyError;
use x11rb::protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt, EventMask, Screen};
use x11rb::protocol::{ErrorKind, Event};

pub struct Concorde<'a, C> {
    connection: &'a C,
    screen: &'a Screen,
    keybinds: Vec<Keybind>,
    layout: Arc<Mutex<WindowNode>>,
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
    pub(crate) fn new(connection: &'a C, screen: &'a Screen, keybinds: Vec<Keybind>) -> Self {
        Self {
            connection,
            screen,
            keybinds,
            layout: Default::default(),
        }
    }

    pub(crate) fn manage_windows(&self) -> Result<(), ReplyError> {
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

    pub(crate) fn setup(&self) -> Result<(), Box<dyn std::error::Error>> {
        let atoms = AtomCollection::new(self.connection)?;
        Ok(())
    }

    pub(crate) fn run(self) -> Result<(), Box<dyn std::error::Error>> {
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
