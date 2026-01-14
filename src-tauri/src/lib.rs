use dirs;
use serde_json::Value;
use std::fs;

#[tauri::command]
fn read_zed_settings() -> Result<Value, String> {
    let mut path =
        dirs::home_dir().ok_or_else(|| "Não foi possível encontrar a pasta Home".to_string())?;

    if cfg!(target_os = "windows") {
        path.push("AppData/Roaming/Zed/settings.json");
    } else {
        path.push(".config/zed/settings.json");
    }

    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;

    let clean_json: Value =
        json5::from_str(&content).map_err(|e| format!("Erro ao processar configurações: {}", e))?;

    Ok(clean_json)
}

#[tauri::command]
fn write_zed_settings(content: String) -> Result<(), String> {
    let mut path =
        dirs::home_dir().ok_or_else(|| "Não foi possível encontrar a pasta Home".to_string())?;

    if cfg!(target_os = "windows") {
        path.push("AppData/Roaming/Zed/settings.json");
    } else {
        path.push(".config/zed/settings.json");
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    std::fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_zed_settings,
            write_zed_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
