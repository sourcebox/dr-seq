//! Common style settings.

use nih_plug_vizia::vizia::prelude::*;

/// Size of the grid cells.
pub const GRID_CELL_SIZE: Units = Units::Pixels(25.0);

/// Spacing of the grid cells.
pub const GRID_CELL_SPACING: Units = Pixels(3.0);

/// Row height of the grid, must be `GRID_CELL_SIZE` + 2 * `GRID_CELL_SPACING`.
pub const GRID_ROW_HEIGHT: Units = Units::Pixels(31.0);

/// Width of additional spacer after columns.
pub const GRID_COL_SPACER_WIDTH: Units = Pixels(3.0);

/// Height of additional spacer between rows.
pub const GRID_ROW_SPACER_HEIGHT: Units = Pixels(3.0);

/// Width of spacer between various elements.
pub const ELEMENT_SPACER_WIDTH: Units = Pixels(10.0);
