extern crate sdl2;

mod phi;
mod views;

use phi::{Events, View, ViewAction, Phi};

fn main() {
    ::phi::spawn("Arcade RS shooter", |phi| {
        Box::new(::views::ShipView::new(phi))
    });
}
