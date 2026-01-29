/// Screen navigation state
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AppScreen {
    #[default]
    Home,
    Dashboard,
    Import,
    ChartConfig,
}
