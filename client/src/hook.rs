use bevy::prelude::*;
use crate::player::Player;
use crate::player_attach::PlayerAttach;

#[derive(Component)]
pub struct Hook {
    pub hook_speed: f32,
    pub hook_max_range: f32,
}

pub fn setup_hook(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a player sprite at position (0, 0) at a higher z-index than map
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(12.0, 26.0)), // Square size 100x100 pixels
            // image: asset_server.load("C:\\Users\\Denise\\Desktop\\Studier\\CINTE\\II1305 Projektkurs\\rustbourn_engines\\client\\assets\\sprites\\top-view\\robot_yellow.png"),
            image: asset_server.load("sprites/top-view/robot_yellow.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
        Hook {
            hook_speed: 500.0,
            hook_max_range: 100.0,
        },
        PlayerAttach {
            offset: Vec2::new(0.0, 20.0), // Offset from player's center
        },
    ));
}

// fn extend_rope(mut query: Query<&mut Transform, With<Hook>>) {
//     for mut transform in query.iter_mut() {
//         transform.scale.y += 5.0;
//     }
// }

pub fn hook_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Sprite, &mut Transform), With<Hook>>,
    time: Res<Time>,
) {
    let growth_rate = 100.0; // Units per second
    let growth_amount = growth_rate * time.delta_secs();
    
    for (mut sprite, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            if let Some(size) = sprite.custom_size {
                sprite.custom_size = Some(Vec2::new(size.x, size.y + growth_amount));
                // Move the sprite up by half the growth amount to maintain bottom position
                transform.translation.y += growth_amount / 2.0;
            }
        }
    }
}