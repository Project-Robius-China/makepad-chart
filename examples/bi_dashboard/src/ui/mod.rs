use makepad_widgets::*;
pub mod screens;
pub mod chart_cell;
pub use screens::AppScreen;
pub use chart_cell::ChartCellRef;

pub const VIEW_SET:&[&[LiveId]] = ids_array!(
    chart_0_0, 
    facebook_button, 
    github_button, 
    gitlab_button, 
    google_button, 
    twitter_button
);