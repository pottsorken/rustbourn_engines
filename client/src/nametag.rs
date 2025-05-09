use bevy::{
    color::palettes::css::*,
    math::ops,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds},
};

use crate::{common::*, db_connection::*, module_bindings::PlayerTableAccess};

use spacetimedb_sdk::{credentials, DbContext, Error, Identity, Table};

#[derive(Component)]
struct AnimateScale;

pub fn spawn_tags(mut commands: Commands) {
    //commands.spawn((
    //    Text2d::new("Lorem Ipsum Dolor"),
    //    TextFont {
    //        font_size: 50.0,
    //        ..default()
    //    },
    //    TextLayout::new_with_justify(JustifyText::Center),
    //    Transform::from_xyz(1.0, 2.0, 3.0),
    //    //Anim
    //    //PlayerAttach {
    //    //    offset: Vec2::new(10., 10.),
    //    //},
    //    //TEXT_JUSTIFICATION.clone(),
    //));

    //commands.spawn((
    //    Text2d::new("Lorem Ipsum Dolor"),
    //    TextFont {
    //        font_size: 10.0,
    //        font_color: TextColor::BLACK,
    //        ..default()
    //    },
    //    TextLayout::new_with_justify(JustifyText::Center),
    //    Transform::from_xyz(50.0, 50.0, 3.0),
    //    PlayerAttach {
    //        offset: Vec2::new(0., -40.),
    //    },
    //));

    // Demonstrate text wrapping
    let slightly_smaller_text_font = TextFont {
        font_size: 35.0,
        ..default()
    };
    let box_size = Vec2::new(150.0, 40.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn((
            Sprite::from_color(Color::rgba(0.25, 0.25, 0.55, 0.5), box_size),
            Transform::from_translation(box_position.extend(30.0)),
            PlayerAttach {
                offset: Vec2::new(0., -40.),
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("Lorem Ipsum Dolor"),
                TextFont {
                    font_size: 15.0,
                    //font_color: TextColor::BLACK,
                    ..default()
                },
                TextColor::BLACK,
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                // Wrap text in the rectangle
                TextBounds::from(box_size),
                // Ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
            ));
        });
}

fn spawn_text(commands: &mut Commands, name: &str) {
    let box_size = Vec2::new(180.0, 40.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn((
            Sprite::from_color(Color::rgba(0.25, 0.25, 0.55, 0.5), box_size),
            Transform::from_translation(box_position.extend(30.0)),
            PlayerAttach {
                offset: Vec2::new(0., -40.),
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("Lorem Ipsum"),
                TextFont {
                    font_size: 15.0,
                    //font_color: TextColor::BLACK,
                    ..default()
                },
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                // Wrap text in the rectangle
                TextBounds::from(box_size),
                TextColor::BLACK,
                // Ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
            ));
        });
}

pub fn spawn_opponent_nametag(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    existing_nametags_query: &Query<&OpponentHook>,
    opponent_id: &Identity,
    local_player_id: &Identity,
    x: f32,
    y: f32,
) {
    // Don't spawn hook for yourself
    if opponent_id == local_player_id {
        return;
    }

    // Don't spawn already existing hooks
    for hook in existing_nametags_query.iter() {
        if hook.id == *opponent_id {
            return; // Hook already exists
        }
    }

    commands.spawn((
        Sprite {
            custom_size: Some(HOOK_CONFIG.hook_size),
            image: asset_server.load(HOOK_CONFIG.hook_path),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(x, y, 5.0),
        OpponentHook { id: *opponent_id },
    ));
}

pub fn update_nametags(
    ctx_wrapper: Res<CtxWrapper>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Sprite, &mut Transform, &OpponentHook), With<OpponentHook>>,
    existing_hooks_query: Query<&OpponentHook>,
    despawn_query: Query<(Entity, &OpponentHook)>,
) {
    let players = ctx_wrapper.ctx.db.player().iter().collect::<Vec<_>>();

    let local_player_id = ctx_wrapper.ctx.identity(); //Get local player's ID

    for player in players {
        let player_id = player.identity;
        spawn_opponent_nametag(
            &mut commands,
            &asset_server,
            &existing_hooks_query,
            &player_id,
            &local_player_id,
            player.hook.position.x,
            player.hook.position.y,
        );

        update_opponent_nametag(
            &mut query,
            &player_id,
            player.hook.position.x,
            player.hook.position.y,
            player.hook.rotation,
        );
    }
}

pub fn update_opponent_nametag(
    query: &mut Query<(&mut Sprite, &mut Transform, &OpponentHook), With<OpponentHook>>,
    id: &Identity,
    x: f32,
    y: f32,
    rotation: f32,
) {
    for (mut sprite, mut transform, hook) in query.iter_mut() {
        if hook.id == *id {
            transform.translation.x = x;
            transform.translation.y = y;
            transform.rotation = Quat::from_rotation_z(rotation).normalize();
        }
    }
}
