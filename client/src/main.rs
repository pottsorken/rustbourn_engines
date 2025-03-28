use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, gather_input) // runs on client
        .add_systems(FixedUpdate, apply_movement) // runs on server
        .run();
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Component)]
struct Obstacle {

}

#[derive(Component, Default)]
struct MoveIntent {
    rotation: f32,
    forward: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.9),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(180.0),
        },
        MoveIntent::default(),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(100.0, 100.0, 0.0),
            ..default()
        },
        Obstacle {
        },
        MoveIntent::default(),
    ));
}


/// Client: Read input and store desired movement direction
fn gather_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MoveIntent, With<Player>>,
) {
    for mut intent in &mut query {
        intent.rotation = 0.0;
        intent.forward = 0.0;

        if keyboard_input.pressed(KeyCode::KeyA) {
            intent.rotation += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            intent.rotation -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            intent.forward += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            intent.forward -= 1.0;
        }
    }
}

/// Server: Apply movement based on client intent
fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Player, &MoveIntent)>,
) {
    for (mut transform, player, intent) in &mut query {
        if intent.rotation != 0.0 {
            transform.rotate_z(intent.rotation * player.rotation_speed * time.delta_secs());
        }

        if intent.forward != 0.0 {
            let forward = transform.rotation * Vec3::Y;
            transform.translation += forward * intent.forward * player.movement_speed * time.delta_secs();
        }
    }
}
