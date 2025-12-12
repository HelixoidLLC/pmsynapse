//! PMSynapse WASM Bindings
//!
//! Browser-compatible bindings for PMSynapse.
//! This provides a lightweight WASM interface for web browsers.
//!
//! Note: This uses standalone implementations rather than snps-core
//! because CozoDB doesn't compile to WASM with full features yet.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// PMSynapse version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Get the version of PMSynapse
#[wasm_bindgen]
pub fn version() -> String {
    VERSION.to_string()
}

/// Initialize PMSynapse in the browser
#[wasm_bindgen]
pub fn init_synapse() -> Result<(), JsValue> {
    // Browser initialization - just return success for now
    Ok(())
}

/// IDLC Item for browser use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlcItem {
    pub id: String,
    pub title: String,
    pub status: String,
    pub created_at: String,
}

impl IdlcItem {
    pub fn new(title: &str) -> Self {
        Self {
            id: format!("idlc_{}", js_sys::Date::now() as u64),
            title: title.to_string(),
            status: "idea".to_string(),
            created_at: js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default(),
        }
    }
}

/// Create a new IDLC item
#[wasm_bindgen]
pub fn create_idlc_item(title: &str) -> Result<String, JsValue> {
    let item = IdlcItem::new(title);
    serde_json::to_string(&item).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// IDLC Stage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlcStage {
    pub id: String,
    pub name: String,
    pub description: String,
}

/// Default IDLC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlcConfig {
    pub team_id: String,
    pub team_name: String,
    pub stages: Vec<IdlcStage>,
}

impl Default for IdlcConfig {
    fn default() -> Self {
        Self {
            team_id: "default".to_string(),
            team_name: "Default Team".to_string(),
            stages: vec![
                IdlcStage {
                    id: "idea".to_string(),
                    name: "Idea".to_string(),
                    description: "Initial idea capture".to_string(),
                },
                IdlcStage {
                    id: "research".to_string(),
                    name: "Research".to_string(),
                    description: "Feasibility research".to_string(),
                },
                IdlcStage {
                    id: "design".to_string(),
                    name: "Design".to_string(),
                    description: "Solution design".to_string(),
                },
                IdlcStage {
                    id: "implementation".to_string(),
                    name: "Implementation".to_string(),
                    description: "Building the solution".to_string(),
                },
                IdlcStage {
                    id: "completion".to_string(),
                    name: "Completion".to_string(),
                    description: "Final delivery".to_string(),
                },
            ],
        }
    }
}

/// Get default IDLC configuration
#[wasm_bindgen]
pub fn get_default_idlc_config() -> Result<String, JsValue> {
    let config = IdlcConfig::default();
    serde_json::to_string(&config).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_create_item() {
        let result = create_idlc_item("Test");
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_default_config() {
        let result = get_default_idlc_config();
        assert!(result.is_ok());
    }
}
