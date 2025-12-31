use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Deserialize, TypePath, Serialize)]
pub struct HouRectMesh {
    pub size: Vec2,
    pub translation: Vec3,
}

#[derive(Resource, Default, Asset, TypePath, Deserialize, Serialize, Debug)]
pub struct HouPlatformData {
    pub rects: Vec<HouRectMesh>,
}

impl HouPlatformData {
    /// Create a new instance with some temporary test data
    pub fn with_temp_data() -> Self {
        Self {
            rects: vec![
                HouRectMesh {
                    size: Vec2::new(10.0, 5.0),
                    translation: Vec3::new(0.0, 0.0, 0.0),
                },
                HouRectMesh {
                    size: Vec2::new(8.0, 4.0),
                    translation: Vec3::new(5.0, 2.0, 1.0),
                },
                HouRectMesh {
                    size: Vec2::new(12.0, 3.0),
                    translation: Vec3::new(-3.0, -1.0, 0.5),
                },
            ],
        }
    }

    /// Export the data to a JSON file
    pub fn export_as_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(self)?;
        fs::write(file_path, json_data)?;
        Ok(())
    }

    /// Import data from a JSON file
    pub fn import_from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_data = fs::read_to_string(file_path)?;
        let data: HouPlatformData = serde_json::from_str(&json_data)?;
        Ok(data)
    }
}
