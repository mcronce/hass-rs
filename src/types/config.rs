use serde::{Deserialize, Serialize};
use std::fmt;

/// This object represents the Home Assistant Config
///
/// This will get a dump of the current config in Home Assistant.
/// [Fetch Config](https://developers.home-assistant.io/docs/api/websocket/#fetching-config)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UnitSystem {
    pub length: String,
    pub mass: String,
    pub pressure: String,
    pub temperature: String,
    pub volume: String,
}

impl fmt::Display for HassConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HassConfig {{\n")?;
        write!(f, "  latitude: {},\n", self.latitude)?;
        write!(f, "  longitude: {},\n", self.longitude)?;
        write!(f, "  elevation: {},\n", self.elevation)?;
        write!(f, "  unit_system: {:?},\n", self.unit_system)?;
        write!(f, "  location_name: {},\n", self.location_name)?;
        write!(f, "  time_zone: {},\n", self.time_zone)?;
        write!(f, "  components: {:?},\n", self.components)?;
        write!(f, "  config_dir: {},\n", self.config_dir)?;
        write!(
            f,
            "  whitelist_external_dirs: {:?},\n",
            self.whitelist_external_dirs
        )?;
        write!(f, "  version: {},\n", self.version)?;
        write!(f, "  config_source: {},\n", self.config_source)?;
        write!(f, "  safe_mode: {},\n", self.safe_mode)?;
        write!(f, "  external_url: {:?},\n", self.external_url)?;
        write!(f, "  internal_url: {:?},\n", self.internal_url)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for UnitSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnitSystem {{\n")?;
        write!(f, "  length: {},\n", self.length)?;
        write!(f, "  mass: {},\n", self.mass)?;
        write!(f, "  pressure: {},\n", self.pressure)?;
        write!(f, "  temperature: {},\n", self.temperature)?;
        write!(f, "  volume: {},\n", self.volume)?;
        write!(f, "}}")?;
        Ok(())
    }
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
    //pub identifiers: Vec<(String, String)>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub name_by_user: Option<String>,
    pub serial_number: Option<String>,
    pub sw_version: Option<String>,
    pub via_device_id: Option<String>,
}
