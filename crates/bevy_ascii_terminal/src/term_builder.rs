use bevy::prelude::{IVec2, UVec2};
use bevy_ascii_terminal::{
    Border, GridPoint, Pivot, Size2d, Terminal, TerminalFont, Tile, TileScaling,
};

/// A builder for creating a terminal.
#[derive(Debug, Clone)]
pub struct TerminalBundleBuilder {
    pub(crate) depth: i32,
    pub(crate) size: UVec2,

    /// Add an auto camera to the terminal.
    pub(crate) auto_camera: bool,

    pub(crate) pos: Option<IVec2>,
    pub(crate) pivot: Option<Pivot>,
    pub(crate) border: Option<Border>,
    pub(crate) clear_tile: Option<Tile>,
    pub(crate) font: Option<TerminalFont>,
    pub(crate) scaling: Option<TileScaling>,
}

impl Default for TerminalBundleBuilder {
    fn default() -> Self {
        Self {
            depth: 0,
            pos: None,
            font: None,
            pivot: None,
            border: None,
            scaling: None,
            clear_tile: None,
            auto_camera: true,
            size: UVec2::new(80, 50),
        }
    }
}

impl TerminalBundleBuilder {
    /// Create a new terminal builder.
    pub fn new(size: impl Size2d) -> Self {
        Self::default().with_size(size)
    }

    /// Add an auto camera to the terminal.
    pub fn with_auto_camera(mut self) -> Self {
        self.auto_camera = true;
        self
    }

    /// Add a border to the terminal.
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    /// Sets the intial z position for the terminal.
    pub fn with_depth(mut self, depth: i32) -> Self {
        self.depth = depth;
        self
    }

    /// Sets the [TileScaling] for the terminal.
    pub fn with_tile_scaling(mut self, scaling: TileScaling) -> Self {
        self.scaling = Some(scaling);
        self
    }

    /// Set the initial size of the terminal.
    pub fn with_size(mut self, size: impl Size2d) -> Self {
        self.size = size.as_uvec2();
        self
    }

    /// Set the initial pivot of the terminal.
    pub fn with_pivot(mut self, pivot: Pivot) -> Self {
        self.pivot = Some(pivot);
        self
    }

    /// Set the initial font of the terminal.
    pub fn with_font(mut self, font: TerminalFont) -> Self {
        self.font = Some(font);
        self
    }

    /// Set the initial position of the terminal.
    pub fn with_position(mut self, pos: impl GridPoint) -> Self {
        self.pos = Some(pos.as_ivec2());
        self
    }

    /// Set the clear tile of the terminal.
    pub fn with_clear_tile(mut self, clear_tile: impl Into<Tile>) -> Self {
        self.clear_tile = Some(clear_tile.into());
        self
    }
}

impl From<TerminalBundleBuilder> for Terminal {
    fn from(builder: TerminalBundleBuilder) -> Self {
        let mut term = Terminal::new(builder.size);

        if let Some(border) = builder.border {
            term = term.with_border(border);
        }

        if let Some(clear_tile) = builder.clear_tile {
            term = term.with_clear_tile(clear_tile);
        }

        term
    }
}
