use bevy::prelude::*;
use rand::Rng;

pub struct Star {
    pub name: String,
    pub date: String,
    pub position: (i64, i64, i64),
}

impl Star {
    // Create a new star instance with a random position
    pub fn new() -> Self {
        Self {
            name: "Unnamed Star".to_string(),
            date: "Unknown Date".to_string(),
            position: (
                rand::thread_rng().gen_range(0..=100),
                rand::thread_rng().gen_range(0..=100),
                rand::thread_rng().gen_range(0..=100),
            ),
        }
    }

    // Function to spawn a sphere at the Star's position
    pub fn spawn_shape(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>) {
        let sphere = meshes.add(Sphere::new(0.2));

        // Spawn the sphere at the Star's position
        commands.spawn(PbrBundle {
            mesh: sphere.clone(),
            transform: Transform::from_translation(Vec3::new(
                self.position.0 as f32,
                self.position.1 as f32, // y is 0 to place the star on the ground plane
                self.position.2 as f32,
            )),
            ..default()
        });
    }
}

// Resource to store all stars
#[derive(Resource, Default)]
pub struct StarList {
    pub stars: Vec<Star>,
}

// System to spawn stars
pub fn spawn_stars(
    mut commands: Commands,
    mut star_list: ResMut<StarList>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Create a new star, add it to the list, and spawn its shape
    let star = Star::new();
    star.spawn_shape(&mut commands, &mut meshes);
    star_list.stars.push(star);
}
