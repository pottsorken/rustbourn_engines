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
            // image: asset_server.load("sprites/top-view/robot_yellow.png"),
            color:Color::srgb(0.8, 0.4, 0.2),
            anchor: bevy::sprite::Anchor::BottomCenter,
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
    mut query: Query<(&mut Sprite, &mut Transform, &Hook)>,
    time: Res<Time>,
) {
    for (mut sprite, mut transform, hook) in query.iter_mut() {
        let growth_rate = hook.hook_speed;
        let growth_amount = growth_rate * time.delta_secs();

        if let Some(size) = sprite.custom_size {
            let mut new_height = size.y;
            let mut y_offset = 0.0;

            if keyboard_input.pressed(KeyCode::Space) {
                // Extend
                if size.y < hook.hook_max_range {
                    new_height = (size.y + growth_amount).min(hook.hook_max_range);
                    y_offset = (new_height - size.y) / 2.0;
                }
            } else {
                // Retract
                if size.y > 0.0 {
                    new_height = (size.y - growth_amount).max(0.0);
                    y_offset = -(size.y - new_height) / 2.0;
                }
            }

            sprite.custom_size = Some(Vec2::new(size.x, new_height));
            
        }
    }
}

