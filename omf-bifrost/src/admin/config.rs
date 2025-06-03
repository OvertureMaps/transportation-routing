use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum HighwayType {
    Trunk,
    TrunkLink,
    Track,
    Footway,
    Pedestrian,
    Bridleway,
    Cycleway,
    Path,
    Motorroad,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AccessMode {
    Auto = 0,
    Pedestrian = 1,
    Bicycle = 2,
    Truck = 3,
    Emergency = 4,
    Taxi = 5,
    Bus = 6,
    Hov = 7,
    Wheelchair = 8,
    Moped = 9,
    Motorcycle = 10,
}

impl AccessMode {
    pub fn bit(self) -> i64 {
        1 << (self as u8)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdminConfig {
    pub allow_intersection_names: HashMap<String, bool>,
    pub admin_access: HashMap<String, HashMap<HighwayType, Vec<AccessMode>>>,
}

impl Default for AdminConfig {
    fn default() -> Self {
        use self::{AccessMode as M, HighwayType as H};

        let allow_intersection_names = HashMap::from([
            ("JP".to_string(), true),
            ("KP".to_string(), true),
            ("KR".to_string(), true),
            ("NI".to_string(), true),
        ]);

        let admin_access = HashMap::from([
            (
                "AU".to_string(),
                HashMap::from([(H::Footway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle])]),
            ),
            (
                "AT".to_string(),
                HashMap::from([
                    (
                        H::Trunk,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::TrunkLink,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (H::Path, vec![M::Pedestrian, M::Wheelchair]),
                ]),
            ),
            (
                "BY".to_string(),
                HashMap::from([
                    (H::Footway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "BE".to_string(),
                HashMap::from([
                    (
                        H::Trunk,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::TrunkLink,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::Track,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair]),
                    (
                        H::Cycleway,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                    (
                        H::Path,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "BR".to_string(),
                HashMap::from([(H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle])]),
            ),
            (
                "CN".to_string(),
                HashMap::from([
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (
                        H::Path,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "DK".to_string(),
                HashMap::from([
                    (
                        H::Trunk,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::TrunkLink,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (H::Track, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "ENG".to_string(),
                HashMap::from([
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "FR".to_string(),
                HashMap::from([
                    (
                        H::Trunk,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::TrunkLink,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (
                        H::Path,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "FI".to_string(),
                HashMap::from([
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "GR".to_string(),
                HashMap::from([
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "HU".to_string(),
                HashMap::from([
                    (
                        H::Trunk,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::TrunkLink,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "IS".to_string(),
                HashMap::from([
                    (H::Footway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "IE".to_string(),
                HashMap::from([(H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle])]),
            ),
            (
                "IT".to_string(),
                HashMap::from([(
                    H::Pedestrian,
                    vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                )]),
            ),
            (
                "NL".to_string(),
                HashMap::from([
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (
                        H::Path,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "NO".to_string(),
                HashMap::from([
                    (
                        H::Track,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                    (H::Footway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "NIR".to_string(),
                HashMap::from([(H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle])]),
            ),
            (
                "OM".to_string(),
                HashMap::from([
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "PH".to_string(),
                HashMap::from([
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (
                        H::Path,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "PL".to_string(),
                HashMap::from([
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "RO".to_string(),
                HashMap::from([
                    (
                        H::Bridleway,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                    (H::Cycleway, vec![M::Bicycle, M::Moped]),
                    (
                        H::Path,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "RU".to_string(),
                HashMap::from([(H::Cycleway, vec![M::Moped, M::Bicycle])]),
            ),
            (
                "TH".to_string(),
                HashMap::from([
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (
                        H::Path,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "TR".to_string(),
                HashMap::from([
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "SCT".to_string(),
                HashMap::from([
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
            (
                "SK".to_string(),
                HashMap::from([
                    (
                        H::Trunk,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::TrunkLink,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                ]),
            ),
            (
                "ES".to_string(),
                HashMap::from([
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (
                        H::Path,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "SE".to_string(),
                HashMap::from([
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair]),
                    (
                        H::Cycleway,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "CH".to_string(),
                HashMap::from([
                    (
                        H::Trunk,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::TrunkLink,
                        vec![M::Auto, M::Truck, M::Bus, M::Hov, M::Taxi, M::Motorcycle],
                    ),
                    (
                        H::Cycleway,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "US".to_string(),
                HashMap::from([
                    (
                        H::Pedestrian,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle],
                    ),
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (
                        H::Path,
                        vec![M::Pedestrian, M::Wheelchair, M::Bicycle, M::Moped],
                    ),
                ]),
            ),
            (
                "WLS".to_string(),
                HashMap::from([
                    (H::Bridleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                    (H::Cycleway, vec![M::Pedestrian, M::Wheelchair, M::Bicycle]),
                ]),
            ),
        ]);
        Self {
            allow_intersection_names,
            admin_access,
        }
    }
}

pub fn load_admin_config(path: Option<&str>) -> Result<AdminConfig> {
    if let Some(path) = path {
        let s = fs::read_to_string(path)
            .with_context(|| format!("Failed to read admin config file '{}'", path))?;
        Ok(serde_json::from_str(&s)
            .with_context(|| format!("Config at '{}' is not valid JSON", path))?)
    } else {
        Ok(AdminConfig::default())
    }
}

pub fn save_default_admin_config(path: &str) -> Result<()> {
    let config = AdminConfig::default();
    let text = serde_json::to_string_pretty(&config)?;
    fs::write(path, text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_access_mode_bit() {
        use AccessMode::*;
        let tests = [
            (Auto, 1),
            (Pedestrian, 2),
            (Bicycle, 4),
            (Truck, 8),
            (Emergency, 16),
            (Taxi, 32),
            (Bus, 64),
            (Hov, 128),
            (Wheelchair, 256),
            (Moped, 512),
            (Motorcycle, 1024),
        ];
        for (mode, expected) in tests {
            assert_eq!(
                mode.bit(),
                expected,
                "mode {:?} should have bit value {}",
                mode,
                expected
            );
        }
    }

    #[test]
    fn test_admin_config_saved_file_is_pretty_json() {
        use std::fs;
        let file = NamedTempFile::new().unwrap();
        save_default_admin_config(file.path().to_str().unwrap()).unwrap();

        let text = fs::read_to_string(file.path()).unwrap();

        // Explicit structure checks
        assert!(text.starts_with("{"), "Should start with a JSON object");
        assert!(
            text.lines().any(|l| l.starts_with("    ")),
            "Should use indentation"
        );
    }

    #[test]
    fn test_admin_config_serialization_roundtrip() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap();

        save_default_admin_config(path).unwrap();

        let loaded_config = load_admin_config(Some(path)).unwrap();
        let default_config = AdminConfig::default();

        assert_eq!(
            loaded_config, default_config,
            "AdminConfig loaded from file should match default"
        );
    }

    #[test]
    fn test_admin_config_load_invalid_file_not_found() {
        let path = "/unlikely/path/that/does/not/exist/config.json";
        let err = load_admin_config(Some(path)).unwrap_err();
        let msg = format!("{:?}", err);
        assert!(
            msg.contains("Failed to read admin config file"),
            "Error should reference file read"
        );
    }

    #[test]
    fn test_admin_config_load_invalid_file_bad_json() {
        let file = NamedTempFile::new().unwrap();
        let mut f = fs::File::create(file.path()).unwrap();
        f.write_all(b"not json").unwrap();
        let err = load_admin_config(Some(file.path().to_str().unwrap())).unwrap_err();
        let msg = format!("{:?}", err);
        assert!(
            msg.contains("is not valid JSON"),
            "Error should reference JSON parse failure"
        );
    }
}
