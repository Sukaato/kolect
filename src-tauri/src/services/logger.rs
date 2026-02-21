use chrono::Local;

fn should_log(level_str: &str) -> bool {
    let log_level = crate::config::LogLevel::from_str(level_str);
    let config_level = crate::config::get_log_level();
    config_level.should_log(&log_level)
}

fn format_message(body: Option<&str>) -> (String, String) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let body_str = body.map_or(String::new(), |b| format!(" | {}", b));
    (timestamp.to_string(), body_str)
}

pub fn debug(message: &str, body: Option<&str>) {
    let (timestamp, body_str) = format_message(body);
    println!("[{}] [DEBUG] {} {}", timestamp, message, body_str);
}

pub fn info(message: &str, body: Option<&str>) {
    let (timestamp, body_str) = format_message(body);
    println!("[{}] [INFO] {} {}", timestamp, message, body_str);
}

pub fn warn(message: &str, body: Option<&str>) {
    let (timestamp, body_str) = format_message(body);
    eprintln!("[{}] [WARN] {} {}", timestamp, message, body_str);
}

pub fn error(message: &str, body: Option<&str>) {
    let (timestamp, body_str) = format_message(body);
    eprintln!("[{}] [ERROR] {} {}", timestamp, message, body_str);
}

pub fn log(level: &str, message: &str, body: Option<&str>) {
    if !should_log(level) {
        return;
    }

    match level.to_lowercase().as_str() {
        "debug" => debug(message, body),
        "info" => info(message, body),
        "warn" => warn(message, body),
        "error" => error(message, body),
        _ => {
            let (timestamp, body_str) = format_message(body);
            println!("[{}] [LOG] {} {}", timestamp, message, body_str);
        }
    }
}
