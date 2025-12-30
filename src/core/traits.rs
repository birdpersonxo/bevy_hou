use bevy::prelude::*;

pub trait HouSpawner: Send + Sync + 'static {
    fn spawn(&self, commands: &mut Commands);
}
