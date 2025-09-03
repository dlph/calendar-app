use std::fs;
use std::sync::Mutex;
use log::debug;
use serde::{Deserialize, Serialize};
use tauri::State;
use tauri::Manager;
use std::path::Path;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .manage(EventStore::default())
        .setup(|app| {
            // Load events on startup
            let store = app.state::<EventStore>();
            match load_events(&app.handle()) {
                Ok(events) => {
                    if let Ok(mut store_events) = store.0.lock() {
                        *store_events = events;
                    }
                }
                Err(e) => {
                    log::warn!("Failed to load events on startup: {}", e);
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_events,
            add_event,
            delete_event,
            get_app_data_path,
            open_file_location
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Event structure
#[derive(Serialize, Deserialize, Clone)]
struct CalendarEvent {
    id: String,
    title: String,
    date: String,
}

// State to hold events
#[derive(Default)]
struct EventStore(Mutex<Vec<CalendarEvent>>);

// Get events
#[tauri::command]
async fn get_events<'r>(app: tauri::AppHandle, store: State<'r, EventStore>) -> Result<Vec<CalendarEvent>, String> {
    let events = store.0.lock().map_err(|e| format!("Lock error: {}", e))?.clone();
    Ok(events)
}


// Add event
#[tauri::command]
async fn add_event<'r>(event: CalendarEvent, app: tauri::AppHandle, store: State<'r, EventStore>) -> Result<(), String> {
    let mut events = store.0.lock().map_err(|e| format!("Lock error: {}", e))?;
    events.push(event.clone());
    save_events(&app, &events)?;
    Ok(())
}

// Delete event
#[tauri::command]
async fn delete_event<'r>(id: String, app: tauri::AppHandle, store: State<'r, EventStore>) -> Result<(), String> {
    let mut events = store.0.lock().map_err(|e| format!("Lock error: {}", e))?;
    events.retain(|e| e.id != id);
    save_events(&app, &events)?;
    Ok(())
}

// Get app data directory path
#[tauri::command]
async fn get_app_data_path(app: tauri::AppHandle) -> Result<String, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Path error: {}", e))?;
    Ok(path.to_string_lossy().to_string())
}

// Open file location in system file manager
#[tauri::command]
async fn open_file_location(path: String) -> Result<(), String> {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file location: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file location: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file location: {}", e))?;
    }

    Ok(())
}

// Save events to file
fn save_events(app: &tauri::AppHandle, events: &[CalendarEvent]) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Path error: {}", e))?;

    // Ensure the directory exists
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).map_err(|e| format!("Create dir error: {}", e))?;
    }

    let path = app_data_dir.join("events.json");
    let json = serde_json::to_string_pretty(events).map_err(|e| format!("Serialization error: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Write error: {}", e))?;
    debug!("Events saved to: {}", path.display());
    Ok(())
}

// Load events from file
fn load_events(app: &tauri::AppHandle) -> Result<Vec<CalendarEvent>, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Path error: {}", e))?
        .join("events.json");
    if !Path::new(&path).exists() {
        return Ok(vec![]);
    }
    let data = fs::read(&path).map_err(|e| format!("Read error: {}", e))?;
    let events: Vec<CalendarEvent> = serde_json::from_slice(&data).map_err(|e| format!("Deserialization error: {}", e))?;
    Ok(events)
}
