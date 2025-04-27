use std::any::Any;

use bevy::prelude::{*, Vec3, Vec2};

use crate::{common::{LastTrackPos, Opponent, OpponentTrack, Track, CtxWrapper, TRACK_CONFIG}, module_bindings::*};
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
        LastTrackPos(Vec2::new(x, y)),
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

pub fn spawn_opponent_track(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    existing_tracks_query: &Query<&OpponentTrack>,
    opponent_id: &Identity,
    local_player_id: &Identity,
    id: u64,
    x: f32,
    y: f32,
    rotation: f32,
    width: f32,
    height: f32,
) {
    // Don't spawn track for yourself
    if opponent_id == local_player_id {
        return;
    }

    /*
    // Don't spawn already existing hooks
    for track in existing_tracks_query.iter() {
        if track.id == id && track.owner_id == *opponent_id {
            return; // Track already exists
        }
    }
    */

    commands.spawn((
        Sprite {
            custom_size: Some(TRACK_CONFIG.size),
            image: asset_server.load(TRACK_CONFIG.path),
            ..default()
        },
        Transform {
            translation: Vec3::new(x, y, 5.0),
            rotation: Quat::from_rotation_z(rotation),
            scale: Vec3::ONE,
        },
        OpponentTrack {
            owner_id: *opponent_id,
            id: id,
            x,
            y,
            rotation,
            width,
            height,
        },
        LastTrackPos(Vec2::new(x, y)),
    ));
}

pub fn spawn_opponent_tracks_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Transform, &mut LastTrackPos, &Opponent)>,
) {
    for (transform, mut last_track_pos, opponent) in query.iter_mut() {
        let forward = transform.rotation * Vec3::Y;
        let right = transform.rotation * Vec3::X;

        let left_offset = transform.translation - right * (TRACK_CONFIG.track_spacing / 2.0);
        let right_offset = transform.translation + right * (TRACK_CONFIG.track_spacing / 2.0);

        let current_pos = transform.translation.truncate();

        if current_pos.distance(last_track_pos.0) >= TRACK_CONFIG.spawn_distance {
            // Spawn left track
            commands.spawn((
                Sprite {
                    custom_size: Some(TRACK_CONFIG.size),
                    image: asset_server.load(TRACK_CONFIG.path),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(left_offset.x, left_offset.y, 1.0),
                    rotation: transform.rotation,
                    scale: Vec3::ONE,
                },
                Track {
                    timer: Timer::from_seconds(TRACK_CONFIG.fade_time, TimerMode::Once),
                },
            ));

            // Spawn right track
            commands.spawn((
                Sprite {
                    custom_size: Some(TRACK_CONFIG.size),
                    image: asset_server.load(TRACK_CONFIG.path),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(right_offset.x, right_offset.y, 1.0),
                    rotation: transform.rotation,
                    scale: Vec3::ONE,
                },
                Track {
                    timer: Timer::from_seconds(TRACK_CONFIG.fade_time, TimerMode::Once),
                },
            ));

            last_track_pos.0 = current_pos;
        }
    }
}


pub fn update_opponent_track(
    query: &mut Query<(&mut Sprite, &mut Transform, &OpponentTrack), With<OpponentTrack>>,
    id: u64,
    x: f32,
    y: f32,
    rotation: f32,
    width: f32,
    height: f32,
) {
    for (mut sprite, mut transform, track) in query.iter_mut() {
        if track.id == id {
            transform.translation.x = x;
            transform.translation.y = y;
            transform.rotation = Quat::from_rotation_z(rotation).normalize();
            sprite.custom_size = Some(bevy::prelude::Vec2::new(width, height));
        }
    }
}
