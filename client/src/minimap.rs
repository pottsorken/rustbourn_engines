use bevy::prelude::*;
use crate::db_connection::{CtxWrapper}; // For querying position from the server

#[derive(Component)]
pub struct Minimap;

pub struct MinimapConfig {
    pub size: f32,          // Size of the minimap
    pub map_radius: f32,    // How far the minimap will show (viewable area)
}

impl Default for MinimapConfig {
    fn default() -> Self {
        MinimapConfig {
            size: 200.0,      // Default minimap size (can adjust as needed)
            map_radius: 500.0, // The radius of the viewable area on the minimap
        }
    }
}

pub fn setup_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<MinimapConfig>,
) {
    // Spawn the minimap background (a circle)
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Minimap)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(config.size, config.size)),
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(0.0, 0.0, 0.0), // Place in the center of the screen
            ..Default::default()
        });

    // Spawn N, W, E, S markers around the minimap
    let markers = [
        ("N", Vec3::new(0.0, config.size / 2.0 - 20.0, 0.0)), // North
        ("S", Vec3::new(0.0, -config.size / 2.0 + 20.0, 0.0)), // South
        ("E", Vec3::new(config.size / 2.0 - 20.0, 0.0, 0.0)), // East
        ("W", Vec3::new(-config.size / 2.0 + 20.0, 0.0, 0.0)), // West
    ];

    for (label, position) in markers.iter() {
        commands.spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: label.to_string(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::BLACK,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
            transform: Transform {
                translation: *position,
                ..Default::default()
            },
            ..Default::default()
        });
    }

    // Add the player marker as a blue dot
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(10.0, 10.0)), // Player dot size
            color: Color::BLUE,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0), // Player at the center
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn update_minimap(
    mut query: Query<(&Transform, &Player)>,
    mut minimap_query: Query<&mut Transform, With<Minimap>>,
    time: Res<Time>,
    config: Res<MinimapConfig>,
    ctx: Res<CtxWrapper>,
) {
    let player_pos = query.single().unwrap().0.translation; // Get the player's world position
    let minimap_transform = minimap_query.single_mut().unwrap();
    
    // Update the minimap position - keeping the player at the center
    let player_on_minimap = Vec2::new(player_pos.x, player_pos.y) / config.map_radius;

    minimap_transform.translation = Vec3::new(player_on_minimap.x, player_on_minimap.y, 0.0);
}
