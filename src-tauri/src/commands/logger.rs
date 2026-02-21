/// Log message from frontend or backend
#[tauri::command]
pub fn log(level: String, message: String, body: Option<serde_json::Value>) -> Result<(), String> {
    let body_str = body.as_ref().map(|b| b.to_string());
    crate::services::logger::log(&level, &message, body_str.as_deref());
    Ok(())
}
