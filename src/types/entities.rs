use serde::Deserialize;
use serde_json::Value;

/// General construct used by HassEntity and HassEvent
#[derive(Debug, Deserialize, PartialEq)]
pub struct Context {
    pub id: String,
    pub parent_id: Option<String>,
    pub user_id: Option<String>,
}

/// This object represents a Home Assistant Entity
///
/// [Entity](https://developers.home-assistant.io/docs/entity_registry_index)
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct HassEntity {
    pub area_id: Option<String>,
    pub config_entry_id: Option<String>,
    pub device_id: Option<String>,
    pub disabled_by: Option<String>,
    pub entity_category: Option<String>,
    pub entity_id: String,
    pub has_entity_name: bool,
    pub hidden_by: Option<String>,
    pub icon: Option<String>,
    pub id: String,
    pub name: Option<String>,
    pub options: serde_json::map::Map<String, serde_json::Value>,
    pub original_name: Option<String>,
    pub platform: String,
    pub translation_key: Option<String>,
    pub unique_id: String,
}

/// This object represents a snapshot of a Home Assistant Entity's state
///
/// [Entity](https://developers.home-assistant.io/docs/core/entity/)
#[derive(Debug, Deserialize, PartialEq)]
pub struct HassEntityState {
    pub entity_id: String,
    pub state: String,
    pub last_changed: String,
    pub last_updated: String,
    pub attributes: Value,
    pub context: Context,
}
