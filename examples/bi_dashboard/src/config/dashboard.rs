use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of charts available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ChartType {
    #[default]
    Bar,
    Line,
    Pie,
    Scatter,
    Radar,
    PolarArea,
    Bubble,
    HorizontalBar,
    Combo,
    Chord,
}

impl ChartType {
    /// Get all available chart types
    pub fn all() -> Vec<ChartType> {
        vec![
            ChartType::Bar,
            ChartType::Line,
            ChartType::Pie,
            ChartType::Scatter,
            ChartType::Radar,
            ChartType::PolarArea,
            ChartType::Bubble,
            ChartType::HorizontalBar,
            ChartType::Combo,
            ChartType::Chord,
        ]
    }

    /// Get display name for chart type
    pub fn display_name(&self) -> &str {
        match self {
            ChartType::Bar => "Bar Chart",
            ChartType::Line => "Line Chart",
            ChartType::Pie => "Pie Chart",
            ChartType::Scatter => "Scatter Plot",
            ChartType::Radar => "Radar Chart",
            ChartType::PolarArea => "Polar Area",
            ChartType::Bubble => "Bubble Chart",
            ChartType::HorizontalBar => "Horizontal Bar",
            ChartType::Combo => "Combo Chart",
            ChartType::Chord => "Chord Diagram",
        }
    }
}

/// Position in the dashboard grid
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize,
}

impl GridPosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

/// Configuration for a single chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    pub id: String,
    pub chart_type: ChartType,
    pub data_table_name: String,
    pub x_column: String,
    pub y_columns: Vec<String>,
    pub title: String,
    pub position: GridPosition,
}

impl ChartConfig {
    /// Create a new chart configuration
    pub fn new(
        chart_type: ChartType,
        data_table_name: String,
        x_column: String,
        y_columns: Vec<String>,
        title: String,
        position: GridPosition,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            chart_type,
            data_table_name,
            x_column,
            y_columns,
            title,
            position,
        }
    }
}

/// Configuration for a dashboard containing multiple charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub id: String,
    pub name: String,
    pub charts: Vec<ChartConfig>,
}

impl DashboardConfig {
    /// Create a new empty dashboard
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            charts: Vec::new(),
        }
    }

    /// Add a chart to the dashboard
    pub fn add_chart(&mut self, chart: ChartConfig) {
        self.charts.push(chart);
    }

    /// Remove a chart by ID
    pub fn remove_chart(&mut self, chart_id: &str) {
        self.charts.retain(|c| c.id != chart_id);
    }

    /// Get a chart by ID
    pub fn get_chart(&self, chart_id: &str) -> Option<&ChartConfig> {
        self.charts.iter().find(|c| c.id == chart_id)
    }

    /// Get a mutable reference to a chart by ID
    pub fn get_chart_mut(&mut self, chart_id: &str) -> Option<&mut ChartConfig> {
        self.charts.iter_mut().find(|c| c.id == chart_id)
    }

    /// Get chart at a specific grid position
    pub fn get_chart_at_position(&self, row: usize, col: usize) -> Option<&ChartConfig> {
        self.charts
            .iter()
            .find(|c| c.position.row == row && c.position.col == col)
    }
}
