<<<<<<< HEAD
use crate::common::{
    CtxWrapper, Leaderboard, LeaderboardEntry, OnMainMenuScreen, Player, PlayerAttach,
    LEADRERBOARD_CONFIG,
};
use crate::module_bindings::PlayerTableAccess;
use crate::start_menu::*;
use bevy::prelude::*;
=======
use bevy::{
    color::palettes::css::*,
    math::ops,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds},
};
>>>>>>> 3903aaf (Added text to the leaderboard, that resizes with leaderboard)
use bevy::render::view::window;
use clap::builder::styling::Style;

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

            commands.spawn((
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
            )).with_children(|builder|{
                builder.spawn((
                    Text2d::new("jarvis the superior overlord"),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor::BLACK,
                    TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                    TextBounds::from(leaderboard_size),
                    Transform::from_translation(Vec3::Z),


                ));
            }) 
            
            ;
            
        }
    }
}

pub fn update_leaderboard_position(
    mut param_set: ParamSet<(
        Query<&Transform, With<Player>>,       // Read-only player transform
        Query<(&mut Transform, &Leaderboard)>, // Mutable leaderboard transform
    )>,
) {
    let leaderboard_offset = Vec2::new(-800.0, 300.0);

    // Get the player's position (read-only)
    if let Ok(player_transform) = param_set.p0().get_single() {
        let player_position = player_transform.translation.xy();

        // Update the leaderboard position
        for (mut leaderboard_transform, _) in param_set.p1().iter_mut() {
            let new_position = player_position + leaderboard_offset;
            leaderboard_transform.translation.x = new_position.x;
            leaderboard_transform.translation.y = new_position.y;
        }
    }
}
