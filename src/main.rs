use bevy::prelude::*;

use rand::Rng;

mod star;
use star::{process_stars, Star};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.2, 0.7, 0.83)))
        .add_systems(Startup, setup)
        .add_systems(Startup, create_stars)
        .add_systems(Update, process_stars)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, -100.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("lightmap/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("lightmap/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 2_000.0,
        },
    ));
}

fn picture(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("photos/bleachhollow.png"),
        transform: Transform {
            scale: Vec3::new(0.3, 0.3, 1.0),
            ..default()
        },
        ..default()
    });
}

fn create_stars(mut meshes: ResMut<Assets<Mesh>>, mut commands: Commands) {
    for _i in 0..=10000 {
        let mut star_new = Star {
            name: "alex".to_string(),
            position: (
                rand::thread_rng().gen_range(-100..=100),
                rand::thread_rng().gen_range(-100..=100),
                rand::thread_rng().gen_range(-100..=100),
            ),

            date: "11/14/24".to_string(),
        };
        star_new.spawn_shape(&mut commands, &mut meshes);
    }
}
