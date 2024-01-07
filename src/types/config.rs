use serde::Deserialize;

/// This object represents the Home Assistant Config
///
/// This will get a dump of the current config in Home Assistant.
/// [Fetch Config](https://developers.home-assistant.io/docs/api/websocket/#fetching-config)
#[derive(Debug, Deserialize, PartialEq)]
pub struct HassConfig {
    pub latitude: f32,
    pub longitude: f32,
    pub elevation: u32,
    pub unit_system: UnitSystem,
    pub location_name: String,
    pub time_zone: String,
    pub components: Vec<String>,
    pub config_dir: String,
    pub whitelist_external_dirs: Vec<String>,
    pub version: String,
    pub config_source: String,
    pub safe_mode: bool,
    pub external_url: Option<String>,
    pub internal_url: Option<String>,
}

/// This is part of HassConfig
#[derive(Debug, Deserialize, PartialEq)]
pub struct UnitSystem {
    pub length: String,
    pub mass: String,
    pub pressure: String,
    pub temperature: String,
    pub volume: String,
}

/// This object represents a Home Assistant Area
///
/// [Area](https://developers.home-assistant.io/docs/area_registry_index)
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct HassArea {
    #[serde(rename = "area_id")]
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub picture: Option<String>,
}

/// This object represents a Home Assistant Device
///
/// [Device](https://developers.home-assistant.io/docs/device_registry_index)
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct HassDevice {
    pub id: String,
    pub name: String,
    pub area_id: Option<String>,
    pub config_entries: Vec<String>,
    pub configuration_url: Option<String>,
    pub connections: Vec<(String, String)>,
    pub disabled_by: Option<String>,
    pub entry_type: Option<String>,
    pub hw_version: Option<String>,
    pub identifiers: Vec<(String, String)>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub name_by_user: Option<String>,
    pub serial_number: Option<String>,
    pub sw_version: Option<String>,
    pub via_device_id: Option<String>,
}
