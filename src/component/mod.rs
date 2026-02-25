// Component module - shared chart components

mod axis;
mod grid;
pub mod legend;
pub mod tooltip;
mod title;
pub mod color_picker;

pub use axis::*;
pub use grid::*;
pub use legend::*;
pub use tooltip::*;
pub use title::*;
pub use color_picker::*;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    legend::live_design(cx);
    tooltip::live_design(cx);
    color_picker::live_design(cx);
}
