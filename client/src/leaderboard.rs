use crate::module_bindings::PlayerTableAccess;
use crate::start_menu::*;
use crate::{
    common::{
        CtxWrapper, Leaderboard, LeaderboardEntry, OnMainMenuScreen, Opponent, Player,
        PlayerAttach, LEADRERBOARD_CONFIG,
    },
    leaderboard,
    module_bindings::LeaderboardTableAccess,
};
use bevy::prelude::*;
use bevy::render::view::window;
use bevy::{
    color::palettes::css::*,
    math::ops,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds},
};
use clap::builder::styling::Style;

use crate::db_connection::{load_leaderboard, update_leaderboard};

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::srgba(0.5, 0.5, 1.0, 0.0);
const SCORE_COLOR: Color = Color::srgba(1.0, 0.5, 0.5, 1.0);

#[derive(Component)]
struct AnimateScale;

pub fn spawn_leaderboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ctx_wrapper: Res<CtxWrapper>,
    query: Query<Entity, With<Leaderboard>>, // Query for existing leaderboard
    player_query: Query<&Transform, With<Player>>,
) {
    if query.is_empty() {
        // Demonstrate text wrapping
        let slightly_smaller_text_font = TextFont {
            font_size: 35.0,
            ..default()
        };

        let leaderboard_size = Vec2::new(200.0, 300.0);
        let leaderboard_offset = Vec2::new(-800.0, 300.0);

        if let Ok(player_transform) = player_query.get_single() {
            let player_position = player_transform.translation.xy();

            let leaderboard_position = player_position + leaderboard_offset;

            // Fetch the leaderboard from the server
            let leaderboard = load_leaderboard(&ctx_wrapper);

            commands
                .spawn((
                    Leaderboard {
                        players: Vec::new(),
                        position: leaderboard_position, // Store position here
                        size: leaderboard_size,         // Store size
                        offset: leaderboard_offset,
                    },
                    Sprite::from_color(Color::rgba(0.25, 0.25, 0.55, 0.5), leaderboard_size),
                    Transform {
                        translation: leaderboard_position.extend(30.0),
                        scale: Vec3::ONE,
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text2d::new("Leaderboard"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor::BLACK,
                        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                        TextBounds::from(leaderboard_size),
                        Transform::from_translation(Vec3::new(
                            0.0,
                            leaderboard_size.y / 2.0 - 30.0,
                            30.0,
                        )),
                    ));

                    for (index, (id, blocks)) in leaderboard.iter().enumerate() {
                        let player_text =
                            format!("{}. Player: {} - Score: {}", index + 1, id, blocks);

                        builder.spawn((
                            Text2d::new(player_text),
                            TextFont {
                                font_size: 15.0, // Slightly smaller font size
                                ..default()
                            },
                            TextColor::BLACK, // Different color for distinction
                            TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                            TextBounds::from(leaderboard_size),
                            // Move the undertext slightly lower than the main text
                            Transform::from_translation(Vec3::new(
                                -10.0,
                                leaderboard_size.y / 2.0 - 80.0 - (index as f32 * 40.0),
                                30.0,
                            )),
                        ));
                    }

                    // If we have fewer than 3 players, display this ---
                    for index in leaderboard.len()..3 {
                        let empty_text = format!("{}. -----", index + 1);

                        builder.spawn((
                            Text2d::new(empty_text),
                            TextFont {
                                font_size: 15.0, // Slightly smaller font size
                                ..default()
                            },
                            TextColor::BLACK, // Different color for distinction
                            TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                            TextBounds::from(leaderboard_size),
                            // Move the undertext slightly lower than the main text
                            Transform::from_translation(Vec3::new(
                                -10.0,
                                leaderboard_size.y / 2.0 - 80.0 - (index as f32 * 40.0),
                                30.0,
                            )),
                        ));
                    }
                });
        }
    }
}

pub fn update_leaderboard_from_db(
    mut param_set: ParamSet<(
        Query<&Transform, With<Player>>,       // Read-only player transform
        Query<(&mut Transform, &Leaderboard)>, // Mutable leaderboard transform
        Query<(&Opponent)>,
    )>,
    ctx_wrapper: Res<CtxWrapper>,
) {
    if let Ok(opponent) = param_set.p2().get_single() {
        let player_id = opponent.id;
        let leaderboard_id = 1;
        println!("[DEBUG] Local player identity: {:?}", player_id);

        update_leaderboard(&ctx_wrapper, player_id, leaderboard_id);
    }
}
