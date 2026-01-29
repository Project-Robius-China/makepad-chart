use serde::{Deserialize, Serialize};

/// Represents a value in a data table cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Number(f64),
    Text(String),
    Empty,
}

impl Value {
    /// Convert value to f64, returning 0.0 for non-numeric values
    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => 0.0,
        }
    }

    /// Convert value to string representation
    pub fn as_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Text(s) => s.clone(),
            Value::Empty => String::new(),
        }
    }
}

/// Represents a table of data with named columns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTable {
    pub name: String,
    pub columns: Vec<String>,
    pub data: Vec<Vec<Value>>,
}

impl DataTable {
    /// Create a new empty data table
    pub fn new(name: String) -> Self {
        Self {
            name,
            columns: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Get the number of rows in the table
    pub fn row_count(&self) -> usize {
        self.data.len()
    }

    /// Get the number of columns in the table
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Get a column index by name
    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c == name)
    }

    /// Get all values from a column by name
    pub fn get_column(&self, name: &str) -> Option<Vec<&Value>> {
        let idx = self.column_index(name)?;
        Some(self.data.iter().map(|row| &row[idx]).collect())
    }

    /// Get a specific cell value
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Value> {
        self.data.get(row)?.get(col)
    }
}
