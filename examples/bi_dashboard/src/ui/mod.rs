pub mod screens;
pub mod chart_cell;
pub mod chart_list;
pub use screens::AppScreen;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    chart_cell::live_design(cx);
    chart_list::live_design(cx);
}