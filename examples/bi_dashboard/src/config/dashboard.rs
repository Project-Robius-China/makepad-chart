use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of charts available (matching ComboChart's DatasetType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ChartType {
    #[default]
    Bar,
    Line,
}

#[allow(dead_code)]
impl ChartType {
    /// Get all available chart types
    pub fn all() -> Vec<ChartType> {
        vec![
            ChartType::Bar,
            ChartType::Line,
        ]
    }

    /// Get display name for chart type
    pub fn display_name(&self) -> &str {
        match self {
            ChartType::Bar => "Bar",
            ChartType::Line => "Line",
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

/// Configuration for a Y-axis column with its chart type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YColumnConfig {
    pub column_name: String,
    pub label: String,
    pub chart_type: ChartType,
    /// RGBA color for the dataset (using `[f32; 4]` for serde compatibility)
    #[serde(default)]
    pub color: Option<[f32; 4]>,
}

impl YColumnConfig {
    pub fn new(column_name: String, label: String, chart_type: ChartType, color: Option<[f32; 4]>) -> Self {
        Self { column_name, label, chart_type, color }
    }
}

/// Configuration for a single chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    pub id: String,
    pub data_table_name: String,
    pub x_column: String,
    pub y_columns: Vec<YColumnConfig>,
    pub title: String,
    pub position: GridPosition,
}

impl ChartConfig {
    /// Create a new chart configuration with per-column chart types
    pub fn new(
        data_table_name: String,
        x_column: String,
        y_columns: Vec<YColumnConfig>,
        title: String,
        position: GridPosition,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
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

    #[allow(dead_code)]
    /// Remove a chart by ID
    pub fn remove_chart(&mut self, chart_id: &str) {
        self.charts.retain(|c| c.id != chart_id);
    }

    #[allow(dead_code)]
    /// Get a chart by ID
    pub fn get_chart(&self, chart_id: &str) -> Option<&ChartConfig> {
        self.charts.iter().find(|c| c.id == chart_id)
    }

    #[allow(dead_code)]
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
