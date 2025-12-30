use bevy::prelude::*;

pub struct HouPlatformPlugin;

impl Plugin for HouPlatformPlugin {
    fn build(&self, _app: &mut App) {
        println!("BevyHouLoaded..")
    }
}
