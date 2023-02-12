use bevy::prelude::{App as BevyApp, Msaa, Plugin};
use bevy_doryen::DoryenBackend;

use crate::prelude::*;

/// The Bevy Roguelike Toolkit plugin.
pub struct BRLTKPlugin {
    /// The Roguelike [`Backend`] to use.
    backend: Box<dyn Backend>,
}

impl Default for BRLTKPlugin {
    #[allow(clippy::box_default)]
    fn default() -> Self {
        Self {
            backend: Box::new(DoryenBackend::default()),
        }
    }
}

impl BRLTKPlugin {
    /// Create a new [`BRLTKPlugin`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`BRLTKPlugin`] with the given [`Backend`].
    pub fn with_backend(backend: impl Backend + 'static) -> Self {
        Self {
            backend: Box::new(backend),
        }
    }
}

impl Plugin for BRLTKPlugin {
    fn build(&self, app: &mut BevyApp) {
        app.insert_resource(Msaa { samples: 1 });

        // Build the backend.
        self.backend.build(app);
    }
}
