use bevy::prelude::*;

use crate::{common::{Opponent, CtxWrapper}, module_bindings::*};
use spacetimedb_sdk::{Identity, Table};

pub fn spawn_opponent(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    query: &Query<(&mut Transform, &Opponent)>,
    id: &Identity,
    x: f32,
    y: f32,
    rotation: f32,
    local_player_id: &Identity,
) {
    if id == local_player_id {
        //println!("Skipping spawn for local player {:?}", id);
        return;
    }

    for (_transf, opp) in &mut query.iter() {
        if opp.id == *id {
            return;
        }
    }
    commands.spawn((
        Sprite {
            custom_size: Some(bevy::prelude::Vec2::new(80.0, 80.0)), // Square size 100x100 pixels
            image: asset_server.load("sprites/top-view/robot_3Dred.png"),
            ..default()
        },
        //TextureAtlas {
        //    layout: asset_server.load("sprites/top-view/robot_3Dblue.png"),
        //    index: 0,
        //}, -- NOTE: If asset-chart is ever used
        Transform::from_xyz(x, y, 2.0)
            .with_scale(bevy::prelude::Vec3::new(1.0, 1.0, 1.0))
            .with_rotation(Quat::from_rotation_z(rotation)),
        Opponent {
            movement_speed: 300.0,                  // meters per second
            rotation_speed: f32::to_radians(180.0), // degrees per second
            id: *id,
        },
    ));
}

pub fn update_opponent(
    query: &mut Query<(&mut Transform, &Opponent)>,
    id: &Identity,
    x: f32,
    y: f32,
    rotation: f32,
) {
    for (mut transform, opponent) in query.iter_mut() {
        if opponent.id == *id {
            transform.translation.x = x;
            transform.translation.y = y;
            transform.rotation = Quat::from_rotation_z(rotation).normalize();
            //let temp_id = opponent.identity.to_u256() % 10_000;
            //if temp_id == 9573 {
            //    print!("{}: {}   ", temp_id, player.position.rotation);
            //}
            //println!("Updated opponent {:?} to position ({}, {})", id, x, y);
        }
    }
}

pub fn despawn_opponents(
    mut commands: Commands,
    ctx_wrapper: Res<CtxWrapper>,
    query: Query<(Entity, &Opponent)>,
) {
    // List all online players
    let online_players: Vec<Identity> = ctx_wrapper
        .ctx
        .db
        .player()
        .iter()
        .map(|player| player.identity)
        .collect();

    for (entity, opponent) in query.iter() {
        if !online_players.contains(&opponent.id) {
            commands.entity(entity).despawn();
        }
    }
}

