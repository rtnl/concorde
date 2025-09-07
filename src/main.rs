mod axis;
mod box2;
pub mod vec2;

use std::process::exit;

use crate::axis::Axis;
use crate::box2::Box2f;
use crate::vec2::Vec2f;
use x11rb::errors::ReplyError;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt, EventMask, Screen},
    protocol::ErrorKind,
};

struct Concorde<'a, C: Connection> {
    connection: &'a C,
}

impl<'a, C: Connection> Concorde<'a, C> {
    // TODO: new
}

fn manage_windows<C>(connection: &C, screen: &Screen) -> Result<(), ReplyError>
where
    C: Connection + ConnectionExt + Send + Sync,
{
    let change = ChangeWindowAttributesAux::default()
        .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY);

    let result = connection
        .change_window_attributes(screen.root, &change)?
        .check();

    if let Err(ReplyError::X11Error(ref error)) = result {
        if error.error_kind == ErrorKind::Access {
            eprintln!("Another window manager is running.");
            exit(1);
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let b = Box2f::new(Vec2f::zero(), Vec2f::new(1920.0, 1080.0));

    let (l, r) = b.split(Axis::Horizontal, 0.25);
    println!("{:?}", l);
    println!("{:?}", r);

    return Ok(());

    let Ok((connection, screen_number)) = x11rb::connect(None) else {
        eprintln!("Failed to connect to X server.");
        exit(1);
    };

    let screen = &connection.setup().roots[screen_number];
    manage_windows(&connection, screen)?;

    loop {
        connection.flush()?;

        let event = connection.wait_for_event()?;
        let mut event_option = Some(event);

        while let Some(ref _event) = event_option {
            // TODO: handle events

            connection.poll_for_event()?;
            event_option = connection.poll_for_event()?;
        }
    }
}
