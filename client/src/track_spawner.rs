
use crate::{
    common::{Player, LastTrackPos, TRACK_CONFIG, OpponentTrack,},
    db_connection::{load_obstacles, CtxWrapper},
    grid::increment_grid_pos,
    opponent,

};
/*use bevy::prelude::*;*/
use bevy::{prelude::*, transform};
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

#[derive(Component)]
pub struct Track {
    pub timer: Timer,
}



pub fn spawn_tracks_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Transform, &mut LastTrackPos), With<Player>>,
) {
    //let track_texture = asset_server.load(TRACK_CONFIG.path);

    for (transform, mut last_track_pos) in query.iter_mut() {
        let forward = transform.rotation * Vec3::Y;
        let right = transform.rotation * Vec3::X;

        let left_offset = transform.translation - right * (TRACK_CONFIG.track_spacing / 2.0);
        let right_offset = transform.translation + right * (TRACK_CONFIG.track_spacing / 2.0);

        let current_pos = transform.translation.truncate();

        if current_pos.distance(last_track_pos.0) >= TRACK_CONFIG.spawn_distance {
            // Spawn a track sprite
            commands.spawn((
                Sprite {
                    custom_size: Some(TRACK_CONFIG.size),
                    image: asset_server.load(TRACK_CONFIG.path),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(left_offset.x, left_offset.y, 1.0),
                    rotation: transform.rotation,
                    scale: Vec3::splat(1.0),
                },
                Track { 
                    timer: 
                    Timer::from_seconds(
                        TRACK_CONFIG.fade_time, 
                        TimerMode::Once
                    )
                },
            ));

            commands.spawn((
                Sprite {
                    custom_size: Some(TRACK_CONFIG.size),
                    image: asset_server.load(TRACK_CONFIG.path),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(right_offset.x, right_offset.y, 1.0),
                    rotation: transform.rotation,
                    scale: Vec3::splat(1.0),
                },
                Track { 
                    timer: 
                    Timer::from_seconds(
                        TRACK_CONFIG.fade_time, 
                        TimerMode::Once
                    )
                },
            ));

            last_track_pos.0 = current_pos;
        }
    }
}

pub fn track_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Track)>,
) {
    for (entity, mut track) in query.iter_mut() {
        track.timer.tick(time.delta());

        if track.timer.finished() {
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
    track_id: u64,

    x: f32,
    y: f32,
    rotation: f32,
) {
    if opponent_id == local_player_id {
        return;
    }

    for track in existing_tracks_query.iter() {
        if track.owner_id == *opponent_id && track.track_id == track_id {
            return; 
        }
    }
    commands.spawn((
        Sprite {
            custom_size: Some(TRACK_CONFIG.size),
            image: asset_server.load(TRACK_CONFIG.path),
            ..default()
        },
        Transform {
            translation: Vec3::new(x, y, 1.0),
            rotation: Quat::from_rotation_z(rotation),
            scale: Vec3::splat(1.0),
        },
        Track {
            timer: Timer::from_seconds(TRACK_CONFIG.fade_time, TimerMode::Once),
        },
        OpponentTrack {
            owner_id: *opponent_id,
            track_id,
        },
    ));
}

pub fn update_opponent_track(
    query: &mut Query<(&mut Transform, &OpponentTrack), With<OpponentTrack>>,
    opponent_id: &Identity,
    track_id: u64,
    x: f32,
    y: f32,
    rotation: f32,
) {
    for (mut transform, track) in query.iter_mut() {
        if track.owner_id == *opponent_id && track.track_id == track_id {
            transform.translation.x = x;
            transform.translation.y = y;
            transform.rotation = Quat::from_rotation_z(rotation);
        }
    }
}