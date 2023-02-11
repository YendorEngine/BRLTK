//! Plugin settings for interloping with `doryen-rs`.
use std::ops::{Deref, DerefMut};

use bevy::prelude::Resource;
use doryen_rs::Console;

use crate::event::Resized;

/// Provides access to the root console of the Doryen engine.
#[derive(Default, Resource)]
pub struct RootConsole(pub(crate) Option<Console>);

impl std::fmt::Debug for RootConsole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RootConsole")
            .field(
                "0",
                if self.0.is_some() { &"<present>" } else { &"<absent>" },
            )
            .finish()
    }
}

impl Deref for RootConsole {
    type Target = Console;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("Inner value should always be set during `update` and `render` phases")
    }
}

impl DerefMut for RootConsole {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().expect("Inner value should always be set during `update` and `render` phases")
    }
}

/// This resource contains the values given by [`fps`](DoryenApi::fps) and
/// [`average_fps`](DoryenApi::average_fps) on the current update tick.
#[derive(Default, Debug, Clone, Copy, Resource)]
pub struct FpsInfo {
    /// The value given by [`fps`](DoryenApi::fps) on the current update tick.
    pub fps: u32,
    /// The value given by [`average_fps`](DoryenApi::average_fps) on the
    /// current update tick.
    pub average_fps: u32,
}

/// How the [`DoryenPlugin`] reacts to the resize event from Doryen.
#[derive(Clone, Copy, Default)]
pub enum ResizeMode {
    /// Do nothing when the window is resized. This is the default behavior.
    Nothing,

    /// Set the console size to match the window size automatically. This
    /// retains the ratio defined between the console size and the screen size
    /// as given in the [`DoryenAppOptions`] at the start of the program.
    #[default]
    Automatic,

    /// Call the given function when the resize event is triggered. Because
    /// Doryen is sensitive to when the root console is resized, the safest
    /// place to make a call to do so and always have the correct behavior is
    /// during this resize callback which comes directly from Doryen itself. The
    /// [`Resized`] event is useful for reacting to resizing within Bevy
    /// systems for other reasons, but will arrive at a point that is too late
    /// to do the root console resizing correctly.
    Callback(fn(&mut RootConsole, Resized)),
}

impl std::fmt::Debug for ResizeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nothing => f.write_str("Nothing"),
            Self::Automatic => f.write_str("Automatic"),
            Self::Callback(_) => f.write_str("Callback"),
        }
    }
}
