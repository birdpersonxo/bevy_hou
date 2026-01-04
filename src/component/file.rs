use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::mesh2d::HouRect;

#[derive(Serialize, Debug, Deserialize)]
pub struct HouLayer {
    pub rect: Option<Vec<HouRect>>,
}

#[derive(Serialize, Debug, Deserialize, Asset, TypePath, Resource)]
pub struct HouData {
    pub layer: HashMap<String, HouLayer>,
}

impl HouData {
    /// Export the data to a JSON file
    pub fn export_as_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(self)?;
        fs::write(file_path, json_data)?;
        Ok(())
    }

    /// Import data from a JSON file
    pub fn import_from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_data = fs::read_to_string(file_path)?;
        let data: HouData = serde_json::from_str(&json_data)?;
        Ok(data)
    }
}
