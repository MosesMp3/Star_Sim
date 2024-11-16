use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone)]
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
        commands
            .spawn(PbrBundle {
                mesh: sphere.clone(),
                transform: Transform::from_translation(Vec3::new(
                    self.position.0 as f32,
                    self.position.1 as f32, // y is 0 to place the star on the ground plane
                    self.position.2 as f32,
                )),
                ..default()
            })
            .insert(self.clone());
    }
}

pub fn process_stars(query: Query<&Star>) {
    for star in query.iter() {
        println!("Star: {} at position: {:?}", star.name, star.position);
    }
}
