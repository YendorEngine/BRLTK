//! `bevy_ascii_terminal` is a Bevy plugin that lets you use `Bevy` with the `bevy_ascii_terminal` plugin.
//!
//! [Bevy]: https://bevyengine.org/
//! [Bevy Ascii Terminal]: https://github.com/sarkahn/bevy_ascii_terminal

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use bevy_ascii_terminal::{AutoCamera, Terminal, TerminalBundle, TerminalPlugin};
use brltk_common::Backend;

mod term_builder;
pub use term_builder::TerminalBundleBuilder;

/// Re-export of the Doryen library types.
pub mod ascii_terminal {
    pub use bevy_ascii_terminal::prelude::*;
}

/// The `ascii_terminal` backend.
#[derive(Default)]
pub struct BevyAsciiTerminalBackend {
    terms: Vec<TerminalBundleBuilder>,
}

impl Backend for BevyAsciiTerminalBackend {
    fn build(&self, app: &mut bevy::app::App) {
        // Add the terminal plugin.
        app.add_plugin(TerminalPlugin);

        // Add the terminals.
        for term_builder in &self.terms {
            let term = Terminal::from((*term_builder).clone());
            let mut bundle = TerminalBundle::from(term);

            bundle = bundle
                .with_depth(term_builder.depth)
                .with_size(term_builder.size);

            if let Some(pivot) = term_builder.pivot {
                bundle = bundle.with_pivot(pivot);
            }

            if let Some(scaling) = term_builder.scaling {
                bundle = bundle.with_tile_scaling(scaling);
            }

            if let Some(font) = &term_builder.font {
                bundle = bundle.with_font(font.clone());
            }

            if let Some(pos) = &term_builder.pos {
                bundle = bundle.with_position(*pos);
            }

            if term_builder.auto_camera {
                app.world.spawn((bundle, AutoCamera));
            } else {
                app.world.spawn(bundle);
            }
        }
    }
}

impl BevyAsciiTerminalBackend {
    /// Create a new terminal builder.
    pub fn with_terminal(mut self, terminal: TerminalBundleBuilder) -> Self {
        self.terms.push(terminal);
        self
    }
}
