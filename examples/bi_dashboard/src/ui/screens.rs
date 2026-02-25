use crate::config::ChartConfig;

/// Screen navigation state
#[derive(Clone, Debug, Default)]
pub enum AppScreen {
    #[default]
    Home,
    DashboardInit,
    DashboardUpdate,
    Import,
    ChartConfig(Option<ChartConfig>),
}
