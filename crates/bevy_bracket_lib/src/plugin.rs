use bevy::utils::{HashMap, HashSet};
use bracket_bevy::{prelude::RGBA, *};
use brltk_common::Backend;

use crate::term::{
    default_gutter_size, SimpleConsoleFeatures, SparseConsoleFeatures, TerminalBuilderFont, TerminalLayer,
};

/// The `bracket-lib` backend.
#[derive(Clone)]
pub struct BracketLibBackend {
    pub(crate) gutter: f32,
    pub(crate) log_diagnostics: bool,
    pub(crate) with_ortho_camera: bool,
    pub(crate) auto_apply_batches: bool,
    pub(crate) layers: Vec<TerminalLayer>,
    pub(crate) with_frame_diagnostics: bool,
    pub(crate) fonts: Vec<TerminalBuilderFont>,
    pub(crate) palette: HashMap<String, RGBA>,
    pub(crate) scaling_mode: TerminalScalingMode,
    pub(crate) with_random_number_generator: bool,
}

impl Default for BracketLibBackend {
    fn default() -> Self {
        Self::empty()
    }
}

impl Backend for BracketLibBackend {
    fn build(&self, app: &mut bevy::app::App) {
        let mut bterm = BTermBuilder::empty()
            .with_gutter(self.gutter)
            .with_timing_log(self.log_diagnostics)
            .with_scaling_mode(self.scaling_mode)
            .with_ortho_camera(self.with_ortho_camera)
            .with_auto_apply_batches(self.auto_apply_batches)
            .with_timing_diagnostics(self.with_frame_diagnostics)
            .with_random_number_generator(self.with_random_number_generator);

        // Build Fonts
        for font in &self.fonts {
            bterm = bterm.with_font(
                &font.filename,
                font.chars_per_row,
                font.n_rows,
                font.font_height_pixels,
            );
        }
        // Build Layers
        for layer in &self.layers {
            bterm = match layer {
                TerminalLayer::Simple {
                    font_index,
                    width,
                    height,
                    features,
                } => {
                    bterm = bterm.with_simple_console(*font_index, *width, *height);
                    if features.contains(&SimpleConsoleFeatures::WithoutBackground) {
                        bterm = bterm.with_background(false);
                    }
                    bterm
                },
                TerminalLayer::Sparse {
                    font_index,
                    width,
                    height,
                    features,
                } => {
                    bterm = bterm.with_sparse_console(*font_index, *width, *height);
                    if features.contains(&SparseConsoleFeatures::WithoutBackground) {
                        bterm = bterm.with_background(false);
                    }
                    bterm
                },
            }
        }

        app.add_plugin(bterm);
    }
}

impl BracketLibBackend {
    /// Create a new empty backend.
    pub fn empty() -> Self {
        Self {
            fonts: Vec::new(),
            layers: Vec::new(),
            log_diagnostics: false,
            with_ortho_camera: true,
            palette: HashMap::new(),
            auto_apply_batches: true,
            with_frame_diagnostics: true,
            gutter: default_gutter_size(),
            with_random_number_generator: false,
            scaling_mode: TerminalScalingMode::Stretch,
        }
    }

    /// Create a new backend with a simple 80x50 console.
    pub fn simple_80x50() -> Self {
        Self {
            log_diagnostics: false,
            with_ortho_camera: true,
            auto_apply_batches: true,
            palette: HashMap::new(),
            with_frame_diagnostics: true,
            gutter: default_gutter_size(),
            with_random_number_generator: false,
            scaling_mode: TerminalScalingMode::Stretch,
            fonts: vec![TerminalBuilderFont::new(
                "terminal_8x8.png",
                16,
                16,
                (8.0, 8.0),
            )],
            layers: vec![TerminalLayer::Simple {
                font_index: 0,
                width: 80,
                height: 50,
                features: HashSet::new(),
            }],
        }
    }

    /// Add simple console to the terminal.
    pub fn with_simple_console(mut self, font_index: usize, width: i32, height: i32) -> Self {
        self.layers.push(TerminalLayer::Simple {
            font_index,
            width,
            height,
            features: HashSet::new(),
        });
        self
    }

    /// Add sparse console to the terminal.
    pub fn with_sparse_console(mut self, font_index: usize, width: i32, height: i32) -> Self {
        self.layers.push(TerminalLayer::Sparse {
            font_index,
            width,
            height,
            features: HashSet::new(),
        });
        self
    }

    /// Set background of previous layer.
    pub fn with_background(mut self, with_background: bool) -> Self {
        if !self.layers.is_empty() {
            let last_index = self.layers.len() - 1;
            match &mut self.layers[last_index] {
                TerminalLayer::Simple { features, .. } => {
                    if with_background {
                        features.remove(&SimpleConsoleFeatures::WithoutBackground);
                    } else {
                        features.insert(SimpleConsoleFeatures::WithoutBackground);
                    }
                },
                TerminalLayer::Sparse { features, .. } => {
                    if with_background {
                        features.remove(&SparseConsoleFeatures::WithoutBackground);
                    } else {
                        features.insert(SparseConsoleFeatures::WithoutBackground);
                    }
                },
            }
        }
        self
    }

    /// Set [`TerminalScalingMode`] for terminal.
    pub fn with_scaling_mode(mut self, scaling_mode: TerminalScalingMode) -> Self {
        self.scaling_mode = scaling_mode;
        self
    }

    /// Add [`bevy::prelude::Camera2d`] to the terminal.
    pub fn with_ortho_camera(mut self, with_ortho_camera: bool) -> Self {
        self.with_ortho_camera = with_ortho_camera;
        self
    }

    /// Add [`bracket_bevy::prelude::RandomNumbers`] to the terminal.
    pub fn with_random_number_generator(mut self, with_random_number_generator: bool) -> Self {
        self.with_random_number_generator = with_random_number_generator;
        self
    }

    /// Add font to the terminal.
    pub fn with_font(
        mut self,
        filename: &str,
        glyphs_per_row: u16,
        rows: u16,
        pixel_size: (f32, f32),
    ) -> Self {
        self.fonts.push(TerminalBuilderFont::new(
            filename,
            glyphs_per_row,
            rows,
            pixel_size,
        ));
        self
    }

    /// Add color palette to the terminal.
    pub fn with_named_color<S: ToString, C: Into<RGBA>>(mut self, name: S, color: C) -> Self {
        self.palette.insert(name.to_string(), color.into());
        self
    }

    /// Add bevy's [`bevy::diagnostic::FrameTimeDiagnosticsPlugin`] palette to the terminal.
    pub fn with_frame_diagnostics(mut self, with_diagnostics: bool) -> Self {
        self.with_frame_diagnostics = with_diagnostics;
        self
    }

    /// Add bevy's [`bevy::diagnostic::LogDiagnosticsPlugin`] palette to the terminal.
    pub fn with_timing_log(mut self, with_diagnostics: bool) -> Self {
        self.log_diagnostics = with_diagnostics;
        self
    }

    /// Auto apply batches to the terminal.
    pub fn with_auto_apply_batches(mut self, with_auto_batches: bool) -> Self {
        self.auto_apply_batches = with_auto_batches;
        self
    }

    /// Add gutter to terminal.
    pub fn with_gutter(mut self, gutter: f32) -> Self {
        self.gutter = gutter;
        self
    }
}
