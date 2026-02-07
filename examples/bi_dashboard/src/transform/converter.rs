use crate::config::YColumnConfig;
use crate::data::{DataTable, Value};
use makepad_charts::{ChartData, Dataset};
use makepad_charts::makepad_widgets::makepad_math::math_f32::vec4;

/// Convert a DataTable to ChartData for visualization
pub fn table_to_chart_data(
    table: &DataTable,
    x_column: &str,
    y_columns: &[String],
) -> Result<ChartData, String> {
    if y_columns.is_empty() {
        return Err("At least one Y column must be specified".to_string());
    }

    // Find column indices
    let x_idx = table
        .column_index(x_column)
        .ok_or_else(|| format!("X column '{}' not found", x_column))?;

    // Extract labels from x column
    let labels: Vec<String> = table
        .data
        .iter()
        .map(|row| match &row[x_idx] {
            Value::Text(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Empty => String::new(),
        })
        .collect();

    // Create chart data with labels
    let mut chart_data = ChartData::new().with_labels(labels);

    // Create datasets for each y column
    for y_col in y_columns {
        let y_idx = table
            .column_index(y_col)
            .ok_or_else(|| format!("Y column '{}' not found", y_col))?;

        let values: Vec<f64> = table
            .data
            .iter()
            .map(|row| row[y_idx].as_number())
            .collect();

        chart_data = chart_data.add_dataset(Dataset::new(y_col).with_data(values));
    }

    Ok(chart_data)
}

/// Convert a DataTable to ChartData with per-column chart types for combo charts
pub fn table_to_combo_chart_data(
    table: &DataTable,
    x_column: &str,
    y_columns: &[YColumnConfig],
) -> Result<ChartData, String> {
    if y_columns.is_empty() {
        return Err("At least one Y column must be specified".to_string());
    }

    // Find column indices
    let x_idx = table
        .column_index(x_column)
        .ok_or_else(|| format!("X column '{}' not found", x_column))?;

    // Extract labels from x column
    let labels: Vec<String> = table
        .data
        .iter()
        .map(|row| match &row[x_idx] {
            Value::Text(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Empty => String::new(),
        })
        .collect();

    // Create chart data with labels
    let mut chart_data = ChartData::new().with_labels(labels);

    // Create datasets for each y column config
    for y_col_config in y_columns {
        let y_idx = table
            .column_index(&y_col_config.column_name)
            .ok_or_else(|| format!("Y column '{}' not found", y_col_config.column_name))?;

        let values: Vec<f64> = table
            .data
            .iter()
            .map(|row| row[y_idx].as_number())
            .collect();

        // Use the label from config, or fall back to column name
        let dataset_label = if y_col_config.label.is_empty() {
            &y_col_config.column_name
        } else {
            &y_col_config.label
        };

        let mut dataset = Dataset::new(dataset_label).with_data(values);
        if let Some([r, g, b, a]) = y_col_config.color {
            dataset = dataset.with_color(vec4(r, g, b, a));
        }
        chart_data = chart_data.add_dataset(dataset);
    }

    Ok(chart_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_to_chart_data() {
        let mut table = DataTable::new("test".to_string());
        table.columns = vec!["Month".to_string(), "Revenue".to_string()];
        table.data = vec![
            vec![Value::Text("Jan".to_string()), Value::Number(100.0)],
            vec![Value::Text("Feb".to_string()), Value::Number(200.0)],
        ];

        let result = table_to_chart_data(&table, "Month", &["Revenue".to_string()]);
        assert!(result.is_ok());

        let chart_data = result.unwrap();
        assert_eq!(chart_data.labels.len(), 2);
        assert_eq!(chart_data.datasets.len(), 1);
    }
}
