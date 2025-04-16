use crate::common::{Player, LastTrackPos, TRACK_CONFIG};
use bevy::prelude::*;

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
        let current_pos = transform.translation.truncate();

        if current_pos.distance(last_track_pos.0) >= TRACK_CONFIG.spawn_distance {
            // Spawn a track sprite
            commands.spawn((
                Sprite {
                    custom_size: Some(TRACK_CONFIG.size),
                    image: asset_server.load(TRACK_CONFIG.path),
                    ..default()
                },
                Transform::from_xyz(current_pos.x, current_pos.y, 1.0),
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