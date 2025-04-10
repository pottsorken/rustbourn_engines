use bevy::{prelude::*, transform};
use crate::{common::{Hook, PlayerAttach, OpponentHook}, db_connection::CtxWrapper, opponent};
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};
use crate::module_bindings::*;




pub fn setup_hook(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a player sprite at position (0, 0) at a higher z-index than map
    commands.spawn((
        Sprite {
            custom_size: Some(bevy::prelude::Vec2::new(12.0, 26.0)), // Square size 100x100 pixels
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
            offset: bevy::prelude::Vec2::new(0.0, 20.0), // Offset from player's center
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
    ctx: Res<CtxWrapper>,
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

            sprite.custom_size = Some(bevy::prelude::Vec2::new(size.x, new_height));
            
            if let Some(player) = ctx.ctx.db.player().identity().find(&ctx.ctx.identity()) {
                ctx.ctx.reducers().update_hook_movement(
                    ctx.ctx.identity(),
                    size.x,
                    new_height,
                ).unwrap();
            }
        }
    }
}

pub fn spawn_opponent_hook(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    existing_hooks_query: &Query<(&OpponentHook)>,
    opponent_id: &Identity,
    local_player_id: &Identity,
    x: f32,
    y: f32,
){
    // Don't spawn hook for yourself
    if opponent_id == local_player_id{
        return;
    }

    // Don't spawn already existing hooks
    for (hook) in existing_hooks_query.iter() {
        if hook.id == *opponent_id {
            return; // Hook already exists
            println!("Works?");
        }
    }

    commands.spawn((
        Sprite {
            custom_size: Some(bevy::prelude::Vec2::new(12.0, 26.0)),
            color: Color::srgb(0.8, 0.4, 0.2), // Opponent hook color
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(x, y, 5.0),
        OpponentHook {
            id: *opponent_id,
        },
    ));

}

pub fn update_opponent_hook(
    query: &mut Query<(&mut Sprite, &mut Transform, &OpponentHook), With<OpponentHook>>,
    id: &Identity,
    x: f32,
    y: f32,
    rotation: f32,
    width: f32,
    height: f32,
){
    for (mut sprite, mut transform, hook) in query.iter_mut(){
        if hook.id == *id{
            transform.translation.x = x;
            transform.translation.y = y;
            transform.rotation = Quat:: from_rotation_z(rotation).normalize();
            sprite.custom_size = Some(bevy::prelude::Vec2::new(width, height));
        }
    }
}

