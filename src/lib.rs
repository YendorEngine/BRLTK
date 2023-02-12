//! Bevy Roguelike Toolkit is an opinionated rougelike toolkit for Bevy.
//!
//! BRLTK lets you use `Bevy` with various roguelike libraries.
//! Currently, it supports [`doryen-rs`] and [`bracket-lib`].
//!
//! Usage:
//! ```no_run
//! use bevy::prelude::App;
//! use brltk::prelude::{
//!     bevy_doryen::{
//!         DoryenAppOptions, DoryenBackend, DoryenInput, RenderSystemExt, RootConsole,
//!         VirtualScanCode,
//!     },
//!     BRLTKPlugin,
//! };
//!
//! const CONSOLE_WIDTH: u32 = 80;
//! const CONSOLE_HEIGHT: u32 = 45;
//!
//! App::new()
//!     .add_plugin(BRLTKPlugin::with_backend(DoryenBackend {
//!         // here are all the available options.
//!         // better practice is to use default values (see other examples)
//!         app_options: DoryenAppOptions {
//!             console_width: CONSOLE_WIDTH,
//!             console_height: CONSOLE_HEIGHT,
//!             screen_width: CONSOLE_WIDTH * 8,
//!             screen_height: CONSOLE_HEIGHT * 8,
//!             window_title: String::from("my roguelike"),
//!             font_path: String::from("terminal_8x8.png"),
//!             vsync: true,
//!             fullscreen: false,
//!             show_cursor: true,
//!             resizable: true,
//!             intercept_close_request: false,
//!             max_fps: 60,
//!         },
//!         ..Default::default()
//!     }))
//!     .add_startup_system(init)
//!     .add_system(input)
//!     .add_doryen_render_system(render)
//!     .run();
//!
//! # fn init() { }
//! # fn input() { }
//! # fn render() { }
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
