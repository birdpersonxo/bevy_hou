// use crate::core::traits::HouSpawner;
use crate::platform::HouPlatformData;
use bevy::prelude::*;

pub fn spawn_rect_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rect_data: Res<HouPlatformData>,
) {
    let quad = meshes.add(Rectangle::from_size(Vec2 { x: 1., y: 1. }));

    for rect in &rect_data.rects {
        commands.spawn((
            Mesh2d(quad.clone()),
            MeshMaterial2d(materials.add(Color::srgb(0.5, 0.2, 0.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
            Transform {
                translation: rect.translation,
                scale: Vec3::new(rect.size.x, rect.size.y, 1.0),
                ..default()
            },
        ));
    }
}
