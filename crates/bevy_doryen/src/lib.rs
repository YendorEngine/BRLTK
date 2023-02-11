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

use crate::engine::DoryenEngine;
use bevy::{ecs::event::ManualEventReader, prelude::App as BevyApp};
use brltk_common::Backend;
use doryen_rs::{App as DoryenApp, Console, MouseButton};

mod engine;
mod event;
mod input;
mod options;
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
    render::*,
};

/// The `doryen-rs` backend.
#[derive(Clone)]
pub struct DoryenBackend {
    /// The [`DoryenAppOptions`] passed to the [`DoryenApp`].
    pub app_options: DoryenAppOptions,

    /// What to do when the Doryen window is resized.
    pub resize_mode: ResizeMode,

    /// Which mouse buttons to request input data for from Doryen during the
    /// input handling.
    /// Defaults to left, middle and right mouse buttons.
    pub mouse_button_listeners: Vec<MouseButton>,
}

impl Backend for DoryenBackend {
    fn build(&self, app: &mut bevy::app::App) {
        // Resources
        app.init_resource::<FpsInfo>().init_resource::<RootConsole>();

        app.add_plugin(crate::event::DoryenEventPlugin)
            .add_plugin(crate::input::DoryenInputPlugin)
            .add_plugin(crate::render::DoryenRenderPlugin);

        let Self {
            app_options,
            resize_mode,
            mouse_button_listeners,
            ..
        } = self.clone();

        app.set_runner(move |app| {
            doryen_runner(
                app,
                app_options.clone(),
                resize_mode,
                mouse_button_listeners.clone(),
            )
        });
    }
}

impl std::fmt::Debug for DoryenBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DoryenPluginSettings")
            .field("app_options", &self.app_options)
            .field("resize_mode", &self.resize_mode)
            .field("mouse_button_listeners", &self.mouse_button_listeners)
            .finish()
    }
}

impl Default for DoryenBackend {
    fn default() -> Self {
        Self {
            resize_mode: ResizeMode::Nothing,
            app_options: DoryenAppOptions::default(),
            mouse_button_listeners: vec![MouseButton::Left, MouseButton::Middle, MouseButton::Right],
        }
    }
}

fn doryen_runner(
    bevy_app: BevyApp,
    app_options: DoryenAppOptions,
    resize_mode: ResizeMode,
    mouse_button_listeners: Vec<MouseButton>,
) {
    let DoryenAppOptions {
        screen_width,
        screen_height,
        console_width,
        console_height,
        ..
    } = app_options;

    let app_options = DoryenAppOptions {
        /// The font path is relative to the `assets` directory.
        font_path: format!("assets/{}", app_options.font_path),
        ..app_options
    };

    println!("Starting Bevy Doryen plugin with the following settings: {app_options:?}");

    DoryenApp::new(app_options)
        .set_engine(Box::new(DoryenEngine {
            bevy_app,
            resize_mode,
            mouse_button_listeners,
            swap_console: Some(Console::new(1, 1)),
            previous_screen_size: (screen_width, screen_height),
            app_exit_event_reader: ManualEventReader::default(),
            previous_console_size: (console_width, console_height),
            set_font_path_event_reader: ManualEventReader::default(),
        }))
        .run();
}
