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

pub fn spawn_tags(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    ctx_wrapper: Res<CtxWrapper>,
    username: Res<Username>,
) {
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

    let player_id = ctx_wrapper.ctx.identity();

    // Find player entity to spawn a child with
    let player_entity = player_query.get_single();

    // Demonstrate text wrapping
    let slightly_smaller_text_font = TextFont {
        font_size: 35.0,
        ..default()
    };
    let box_size = Vec2::new(150.0, 40.0);
    let box_position = Vec2::new(0.0, -50.0);

    let binding = commands
        .spawn((
            Sprite::from_color(Color::rgba(0.55, 0.55, 0.55, 0.5), box_size),
            Transform::from_translation(box_position.extend(30.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new(username.into_inner().name.clone()),
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
        })
        .id();
    let nametag = binding;

    commands
        .entity(player_entity.expect("Could not find player entity"))
        .add_children(&[nametag]);
}

pub fn spawn_opponent_nametag(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    existing_nametags_query: &Query<&OpponentNametag>,
    opponent_query: &Query<(Entity, &Opponent)>,
    opponent_id: &Identity,
    local_player_id: &Identity,
    player_name: String,
    x: f32,
    y: f32,
) {
    // Don't spawn hook for yourself
    if opponent_id == local_player_id {
        return;
    }

    // If name is unset
    if player_name == "Lorem Ipsum" {
        return;
    }

    // Don't spawn already existing hooks
    for hook in existing_nametags_query.iter() {
        if hook.id == *opponent_id {
            return; // Nametag already exists
        }
    }
    spawn_text(&mut commands, &player_name, opponent_id, &opponent_query);
}

pub fn update_nametags_content(
    ctx_wrapper: Res<CtxWrapper>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // TODO: Add query for opponents, with their server ids, and
    // search for opponentid->entity
    mut opponent_query: Query<(Entity, &Opponent)>,
    mut query: Query<(&mut Sprite, &mut Transform, &OpponentNametag), With<OpponentNametag>>,
    existing_nametags_query: Query<&OpponentNametag>,
    despawn_query: Query<(Entity, &OpponentNametag)>,
) {
    let players = ctx_wrapper.ctx.db.player().iter().collect::<Vec<_>>();

    let local_player_id = ctx_wrapper.ctx.identity(); //Get local player's ID

    for player in players {
        let player_id = player.identity;
        let player_name = player.name.clone();
        spawn_opponent_nametag(
            &mut commands,
            &asset_server,
            &existing_nametags_query,
            &opponent_query,
            &player_id,
            &local_player_id,
            player_name,
            player.hook.position.x,
            player.hook.position.y,
        );

        //update_opponent_nametag(
        //    &mut query,
        //    &player_id,
        //    player.hook.position.x,
        //    player.hook.position.y,
        //    player.hook.rotation,
        //);
    }
}

pub fn update_nametags_positions(
    mut query: Query<(&mut Sprite, &mut Transform, &OpponentNametag), With<OpponentNametag>>,
    ctx_wrapper: Res<CtxWrapper>,
    //query: &mut Query<(&mut Sprite, &mut Transform, &OpponentNametag), With<OpponentNametag>>,
    //id: &Identity,
    //x: f32,
    //y: f32,
    //rotation: f32,
) {
    let players = ctx_wrapper.ctx.db.player().iter().collect::<Vec<_>>();

    let local_player_id = ctx_wrapper.ctx.identity(); //Get local player's ID

    //for player in players {
    //    let player_id = player.identity;
    //    for (mut sprite, mut transform, hook) in query.iter_mut() {
    //        if hook.id == player_id {
    //            transform.translation.x = player.hook.position.x; //x;
    //            transform.translation.y = player.hook.position.y; //y;
    //            transform.rotation = Quat::from_rotation_z(player.hook.rotation).normalize();
    //        }
    //    }
    //}
}
fn spawn_text(
    commands: &mut Commands,
    name: &str,
    opponent_id: &Identity,
    opponent_query: &Query<(Entity, &Opponent)>,
) {
    // Find opponent entity to spawn a child with
    let mut opponent_entity = None;
    //let mut opponent_name = None;
    for (entity, opponent) in opponent_query.iter() {
        if opponent.id == *opponent_id {
            opponent_entity = Some(entity);
            //opponent_name = Some(opponent.id());
        }
    }

    // Specify size of box
    let box_size = Vec2::new(180.0, 40.0);
    let box_position = Vec2::new(0.0, -50.0);

    let binding = commands
        .spawn((
            Sprite::from_color(Color::rgba(0.85, 0.05, 0.05, 0.5), box_size),
            Transform::from_translation(box_position.extend(30.0)),
            OpponentNametag {
                id: opponent_id.clone(),
            }, // To not double spawn

               //PlayerAttach {
               //    offset: Vec2::new(0., -40.),
               //},
        ))
        .with_children(|builder| {
            builder.spawn((
                //Text2d::new(name),
                Text2d::new(name),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                TextBounds::from(box_size),
                TextColor::from(Color::srgb(0.99, 0., 0.)),
                Transform::from_translation(Vec3::Z),
            ));
        })
        .id();
    let nametag = binding;

    commands
        .entity(opponent_entity.expect("Did not find opponent entity from online id"))
        .add_children(&[nametag]);
}
