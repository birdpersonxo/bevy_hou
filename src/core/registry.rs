use bevy::prelude::*;

use crate::core::traits::HouSpawner;

#[derive(Resource, Default)]
pub struct HouSpawnerRegistry {
    pub spawners: Vec<Box<dyn HouSpawner>>,
}

impl HouSpawnerRegistry {
    pub fn register(&mut self, spawner: impl HouSpawner) {
        self.spawners.push(Box::new(spawner));
    }
}
