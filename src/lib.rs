pub mod indicators;
pub mod models;
pub mod scoring;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn analyze_text(text: &str, max_chars: usize) -> String {
    let text = text.trim();

    if text.is_empty() {
        return serde_json::json!({"error": "Text cannot be empty"}).to_string();
    }

    if text.len() > max_chars {
        return serde_json::json!({
            "error": format!("Text too long ({} chars, max {})", text.len(), max_chars)
        })
        .to_string();
    }

    let (breakdown, flagged_phrases) = indicators::analyze_indicators(text);
    let config = models::DetectorConfig::default();
    let result = scoring::create_detection_result(text, breakdown, flagged_phrases, &config);

    serde_json::to_string(&result).unwrap_or_else(|error| {
        serde_json::json!({"error": format!("Serialization error: {}", error)}).to_string()
    })
}
