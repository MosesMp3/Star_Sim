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



// start creating the resource of the vector also might have todo that with the vars that are 0





pub fn process_stars(
    query: Query<&Star>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &Camera)>,
    time: Res<Time>,
) {
    let mut star_list = vec![];
    let mut runonce = Local::new(0);
    let mut starnumber = Local::
    if runonce == 0 {
        for (index, star) in query.iter().enumerate() {
            star_list.push((index as i64, star.position));
        }
        runonce += 1
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        starnumber -= 1;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        starnumber += 1;
    }
    for (mut transform, _camera) in camera_query.iter_mut() {
        transform.translation.x = star_list[starnumber].1 .0 as f32;
        println!("{starnumber}");
    }
}
