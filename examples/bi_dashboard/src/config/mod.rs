pub mod dashboard;
pub mod persistence;

pub use dashboard::{ChartConfig, ChartType, DashboardConfig, YColumnConfig};
pub use persistence::{load_all_dashboards, save_dashboard};
