//! `bevy_bracket_lib` is a Bevy plugin that lets you use `Bevy` with the `bracket_bevy`
//! roguelike library.
//!
//! [Bevy]: https://bevyengine.org/
//! [Bracket-lib]: https://github.com/amethyst/bracket-lib

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod plugin;
mod term;

pub use bracket_bevy::{prelude::*, *};
pub use bracket_pathfinding::prelude::*;

pub use crate::plugin::*;
