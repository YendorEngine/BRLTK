//! Bevy Roguelike Toolkit is an opinionated rougelike toolkit for Bevy.
//!
//! BRLTK lets you use [`Bevy`] with various roguelike libraries.
//! Currently, it supports [`Doryen`] and [`bracket-lib`].
//!
//! Usage:
//! ```no_run
//! ```
//!
//! [Bevy]: https://bevyengine.org/
//! [Doryen]: https://github.com/jice-nospam/doryen-rs
//! [Bracket-lib]: https://github.com/amethyst/bracket-lib

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod plugin;

/// Crate prelude.
pub mod prelude {
    #[cfg(feature = "bevy_bracket_lib")]
    pub use bevy_bracket_lib;
    #[cfg(feature = "bevy_doryen")]
    pub use bevy_doryen;
    pub use brltk_common::*;
    // Yendor
    #[cfg(feature = "yendor_lib")]
    pub use yendor_lib::prelude::*;

    // Internal Prelude
    pub use crate::plugin::*;
}
