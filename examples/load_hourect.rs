use bevy::{prelude::*, window::WindowResolution};
use bevy_hou::{HouAssetLoader, HouData};

const WIDHT_FACTOR: u32 = 4;
const WIN_WIDTH: u32 = 4096 / WIDHT_FACTOR;
const WIN_HEIGHT: u32 = 2642 / WIDHT_FACTOR;
fn main() {
    println!("APP WIN SIZE: {} x {}", WIN_WIDTH, WIN_HEIGHT);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Hou".to_string(),
                resolution: WindowResolution::new(WIN_WIDTH, WIN_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_asset::<HouData>()
        .init_asset_loader::<HouAssetLoader>()
        .add_systems(Startup, (load_asset, setup_camera))
        .add_systems(Update, spawn_hou_platform)
        .run();
}

#[derive(Resource, Default)]
pub struct State {
    pub hou_handle: Handle<HouData>,
    pub hou_spawned: bool,
}

fn load_asset(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.hou_handle = asset_server.load("data/platform2.json");
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_hou_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<State>,
    hou_data: Res<Assets<HouData>>,
) {
    if state.hou_spawned {
        return;
    }

    // Get the asset if it's loaded
    let Some(hou_asset) = hou_data.get(&state.hou_handle) else {
        info!("Hou Platform Data not loaded yet");
        return;
    };

    let colors = [
        Color::srgb(3., 0.2, 0.2),  // Red
        Color::srgb(0.2, 0.8, 0.2), // Green
        Color::srgb(0.2, 0.2, 0.8), // Blue
        Color::srgb(0.8, 0.8, 0.2), // Yellow
        Color::srgb(0.8, 0.2, 0.8), // Purple
        Color::srgb(0.2, 0.8, 0.8), // Cyan
    ];

    for (_name, layer) in hou_asset.layer.iter() {
        match &layer.rect {
            Some(mesh2d) => {
                for (i, rect) in mesh2d.iter().enumerate() {
                    let rect = rect.to_mesh();
                    let color = colors[i % colors.len()];
                    let mesh_handle = meshes.add(rect);

                    commands.spawn((
                        // rendering
                        Mesh2d::from(mesh_handle),
                        MeshMaterial2d(materials.add(ColorMaterial {
                            color: color,
                            ..default()
                        })),
                        // Transform::from_translation(Vec3::Z * 0.1), // Slightly above other entities
                    ));
                }
            }
            _ => {}
        };
    }

    state.hou_spawned = true;
}
