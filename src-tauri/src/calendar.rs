use log::debug;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::Manager;

// Event structure
#[derive(Serialize, Deserialize, Clone)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub date: String,
}

// Save events to file
pub fn save_events(app: &tauri::AppHandle, events: &[CalendarEvent]) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Path error: {}", e))?;

    // Ensure the directory exists
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).map_err(|e| format!("Create dir error: {}", e))?;
    }

    let path = app_data_dir.join("events.json");
    let json =
        serde_json::to_string_pretty(events).map_err(|e| format!("Serialization error: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Write error: {}", e))?;
    debug!("Events saved to: {}", path.display());
    Ok(())
}

// Load events from file
pub fn load_events(app: &tauri::AppHandle) -> Result<Vec<CalendarEvent>, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Path error: {}", e))?
        .join("events.json");
    if !Path::new(&path).exists() {
        return Ok(vec![]);
    }
    let data = fs::read(&path).map_err(|e| format!("Read error: {}", e))?;
    let events: Vec<CalendarEvent> =
        serde_json::from_slice(&data).map_err(|e| format!("Deserialization error: {}", e))?;
    Ok(events)
}
