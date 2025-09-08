mod axis;
mod box2;
pub mod vec2;

use std::process::exit;

use crate::axis::Axis;
use crate::box2::Box2f;
use crate::vec2::Vec2f;

use x11rb::errors::ReplyError;
use x11rb::protocol::xproto::KeyButMask;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt, EventMask, Screen},
    protocol::{ErrorKind, Event},
};

struct Concorde<'a, C: Connection> {
    connection: &'a C,
    screen: &'a Screen,
    layout: Box2f,
}

impl<'a, C: Connection> Concorde<'a, C> {
    fn new(connection: &'a C, screen: &'a Screen) -> Self {
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
        // TODO: more atoms
        let wm_protocols = self.connection.intern_atom(false, b"WM_PROTOCOLS")?;
        println!("{:?}", wm_protocols.raw_reply());

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
                        // Q
                        if event.detail == 24 && event.state == KeyButMask::SHIFT | KeyButMask::MOD4
                        {
                            return Ok(());
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

    let concorde = Concorde::new(&connection, &connection.setup().roots[screen_number]);

    concorde.manage_windows()?;
    concorde.setup()?;
    concorde.run()
}
