use bevy::utils::HashSet;

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(crate) fn default_gutter_size() -> f32 {
    // Testing showed that an 8-pixel gutter is enough to fix
    // Big Sur and Windows 11.
    8.0
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub(crate) fn default_gutter_size() -> f32 {
    // Testing showed that an 8-pixel gutter is enough to fix
    // Big Sur and Windows 11.
    0.0
}

#[derive(Clone, Debug)]
pub(crate) struct TerminalBuilderFont {
    pub(crate) n_rows: u16,
    pub(crate) filename: String,
    pub(crate) chars_per_row: u16,
    pub(crate) font_height_pixels: (f32, f32),
}

impl TerminalBuilderFont {
    pub(crate) fn new<S: ToString>(
        image_filename: S,
        chars_per_row: u16,
        n_rows: u16,
        font_height_pixels: (f32, f32),
    ) -> Self {
        Self {
            filename: image_filename.to_string(),
            chars_per_row,
            n_rows,
            font_height_pixels,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TerminalLayer {
    Simple {
        font_index: usize,
        width: i32,
        height: i32,
        features: HashSet<SimpleConsoleFeatures>,
    },
    Sparse {
        font_index: usize,
        width: i32,
        height: i32,
        features: HashSet<SparseConsoleFeatures>,
    },
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SimpleConsoleFeatures {
    WithoutBackground,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SparseConsoleFeatures {
    WithoutBackground,
}
