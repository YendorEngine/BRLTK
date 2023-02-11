//! Events for handling `doryten-rs` events.

use std::borrow::Cow;

use bevy::prelude::{App, Plugin};

/// Event plugin for handling `doryten-rs` events.
pub struct DoryenEventPlugin;
impl Plugin for DoryenEventPlugin {
    fn build(&self, app: &mut App) { app.add_event::<SetFontPath>().add_event::<Resized>(); }
}

/// When you want to change Doryen's font path, emit an event of this type.
/// bevy_doryen will call [`set_font_path`](DoryenApi::set_font_path) with the
/// provided value.
#[derive(Debug, Clone)]
pub struct SetFontPath(pub Cow<'static, str>);

/// Resized event object. Whenever Doryen's [`resize`](Engine::resize) method is
/// called, an event of this type is emitted.
#[derive(Debug, Clone, Copy)]
pub struct Resized {
    /// The previous width of the Doryen game window.
    pub previous_width: u32,
    /// The previous height of the Doryen game window.
    pub previous_height: u32,
    /// The new width of the Doryen game window.
    pub new_width: u32,
    /// The new height of the Doryen game window.
    pub new_height: u32,
}
