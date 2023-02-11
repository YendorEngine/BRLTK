//! bevy_doryen is a Bevy plugin that lets you use [`Bevy`] with the [`Doryen`]
//! roguelike library.
//!
//! Usage:
//! ```no_run
//! # use bevy_app::App;
//! # use bevy_doryen::{
//! #     DoryenPluginSettings,
//! #     DoryenPlugin,
//! #     RenderSystemExtensions,
//! #     ResizeMode,
//! #     MouseButton
//! # };
//! # use bevy_doryen::doryen::DoryenAppOptions;
//! # use bevy_ecs::system::IntoSystem;
//! App::build()
//!     // Insert a `DoryenPluginSettings` resource to configure the plugin.
//!     .insert_resource(DoryenPluginSettings {
//!         // `app_options` lets you configure Doryen just as if you were
//!         // using Doryen without Bevy. The default is `DoryenAppOptions::default()`.
//!         app_options: DoryenAppOptions {
//!             show_cursor: true,
//!             resizable: true,
//!             ..DoryenAppOptions::default()
//!         },
//!         // Lets you configure which mouse buttons to listen for. The default
//!         // is left, middle and right click.
//!         mouse_button_listeners: vec![
//!             MouseButton::Left,
//!             MouseButton::Middle,
//!             MouseButton::Right,
//!         ],
//!         // Lets you configure how the application should behave when resized.
//!         // The default is `ResizeMode::Nothing`. See `ResizeMode`'s
//!         // documentation for more information.
//!         resize_mode: ResizeMode::Nothing
//!     })
//!     // Add the `DoryenPlugin` to Bevy.
//!     .add_plugin(DoryenPlugin)
//!     // Add your Bevy systems like usual. Excluding startup systems, which
//!     // only run once, these systems are run during Doryen's update phase;
//!     // i.e. 60 times per second.
//!     .add_startup_system(init.system())
//!     .add_system(input.system())
//!     // The `RenderSystemExtensions` trait lets you add systems that should
//!     // be run during Doryen's render phase.
//!     .add_doryen_render_system(render.system())
//! .run();
//!
//! # fn init() { }
//! # fn input() { }
//! # fn render() { }
//! ```
//!
//! [Bevy]: https://bevyengine.org/
//! [Doryen]: https://github.com/jice-nospam/doryen-rs

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod engine;
mod event;
mod input;
mod options;
mod plugin;
mod render;

/// Re-export of the Doryen library types.
pub mod doryen {
    pub use doryen_rs::*;
}

/// Crate prelude.
pub use crate::{
    doryen::{AppOptions as DoryenAppOptions, ScanCode as VirtualScanCode},
    event::*,
    input::*,
    options::*,
    plugin::*,
    render::*,
};
