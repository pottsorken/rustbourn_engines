use bevy::prelude::*;

#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn a player sprite at position (0, 0)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.9), // Blue-green color
                custom_size: Some(Vec2::new(100.0, 100.0)), // Square size 100x100 pixels
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Player {
            movement_speed: 500.0,                  // meters per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
        }
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut query {
        // Handle rotation with A/D keys
        let mut rotation_dir = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_dir += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_dir -= 1.0;
        }
        
        // Apply rotation
        if rotation_dir != 0.0 {
            transform.rotate_z(rotation_dir * player.rotation_speed * time.delta_secs());
        }

        // Handle movement with W/S keys (forward/backward relative to rotation)
        let mut move_dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            move_dir.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            move_dir.y -= 1.0;
        }

        // Apply movement relative to player's rotation
        if move_dir != Vec3::ZERO {
            let move_direction = transform.rotation * move_dir.normalize();
            transform.translation += move_direction * player.movement_speed * time.delta_secs();
        }
    }
}
