use super::table::{DataTable, Value};
use csv::ReaderBuilder;
use std::path::Path;

/// Parse a CSV file and return a DataTable
pub fn parse_csv(path: &str) -> Result<DataTable, String> {
    // Verify file exists
    if !Path::new(path).exists() {
        return Err(format!("File not found: {}", path));
    }

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .map_err(|e| format!("Failed to open CSV: {}", e))?;

    // Get headers
    let headers = reader
        .headers()
        .map_err(|e| format!("Failed to read headers: {}", e))?
        .iter()
        .map(|h| h.to_string())
        .collect::<Vec<_>>();

    // Parse rows
    let mut data = Vec::new();
    for (idx, result) in reader.records().enumerate() {
        let record = result.map_err(|e| format!("Parse error at row {}: {}", idx + 1, e))?;

        let row = record
            .iter()
            .map(parse_value)
            .collect::<Vec<_>>();

        // Verify row length matches header count
        if row.len() != headers.len() {
            return Err(format!(
                "Row {} has {} columns but expected {}",
                idx + 1,
                row.len(),
                headers.len()
            ));
        }

        data.push(row);
    }

    // Extract filename for table name
    let name = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("data")
        .to_string();

    Ok(DataTable {
        name,
        columns: headers,
        data,
    })
}

/// Parse a field value, attempting to detect type
fn parse_value(s: &str) -> Value {
    let trimmed = s.trim();

    if trimmed.is_empty() {
        return Value::Empty;
    }

    // Try to parse as number
    if let Ok(num) = trimmed.parse::<f64>() {
        return Value::Number(num);
    }

    // Otherwise treat as text
    Value::Text(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert!(matches!(parse_value("123"), Value::Number(_)));
        assert!(matches!(parse_value("123.45"), Value::Number(_)));
        assert!(matches!(parse_value("text"), Value::Text(_)));
        assert!(matches!(parse_value(""), Value::Empty));
        assert!(matches!(parse_value("  "), Value::Empty));
    }
}
