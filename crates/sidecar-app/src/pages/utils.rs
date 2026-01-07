use serde_json::Value;

pub fn strip_str(str: &str) -> String {
    str.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn format_json(str: &str) -> anyhow::Result<String, String> {
    let val: Value = serde_json::from_str(str).map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&val).map_err(|e| e.to_string())
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}
