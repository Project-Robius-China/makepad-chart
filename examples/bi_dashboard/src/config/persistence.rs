use super::dashboard::DashboardConfig;
use std::fs;
use std::path::Path;

/// Directory where dashboards are saved
const DASHBOARDS_DIR: &str = "dashboards";

/// Save a dashboard configuration to a JSON file
pub fn save_dashboard(dashboard: &DashboardConfig) -> Result<(), String> {
    // Create dashboards directory if it doesn't exist
    fs::create_dir_all(DASHBOARDS_DIR)
        .map_err(|e| format!("Failed to create dashboards directory: {}", e))?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(dashboard)
        .map_err(|e| format!("Failed to serialize dashboard: {}", e))?;

    // Write to file
    let filename = format!("{}/{}.json", DASHBOARDS_DIR, dashboard.id);
    fs::write(&filename, json).map_err(|e| format!("Failed to write dashboard file: {}", e))?;

    Ok(())
}

#[allow(dead_code)]
/// Load a dashboard configuration from a JSON file
pub fn load_dashboard(id: &str) -> Result<DashboardConfig, String> {
    let filename = format!("{}/{}.json", DASHBOARDS_DIR, id);

    if !Path::new(&filename).exists() {
        return Err(format!("Dashboard file not found: {}", filename));
    }

    let json = fs::read_to_string(&filename)
        .map_err(|e| format!("Failed to read dashboard file: {}", e))?;

    let dashboard: DashboardConfig = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse dashboard JSON: {}", e))?;

    Ok(dashboard)
}

/// Load all dashboard configurations from the dashboards directory
pub fn load_all_dashboards() -> Result<Vec<DashboardConfig>, String> {
    // Create dashboards directory if it doesn't exist
    if !Path::new(DASHBOARDS_DIR).exists() {
        fs::create_dir_all(DASHBOARDS_DIR)
            .map_err(|e| format!("Failed to create dashboards directory: {}", e))?;
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(DASHBOARDS_DIR)
        .map_err(|e| format!("Failed to read dashboards directory: {}", e))?;

    let mut dashboards = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let json = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

            if let Ok(dashboard) = serde_json::from_str::<DashboardConfig>(&json) {
                dashboards.push(dashboard);
            }
        }
    }

    Ok(dashboards)
}

#[allow(dead_code)]
/// Delete a dashboard configuration file
pub fn delete_dashboard(id: &str) -> Result<(), String> {
    let filename = format!("{}/{}.json", DASHBOARDS_DIR, id);

    if !Path::new(&filename).exists() {
        return Err(format!("Dashboard file not found: {}", filename));
    }

    fs::remove_file(&filename).map_err(|e| format!("Failed to delete dashboard file: {}", e))?;

    Ok(())
}
