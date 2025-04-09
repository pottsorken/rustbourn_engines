use bevy::prelude::*;

use crate::{common::{Player, PlayerAttach}, db_connection::CtxWrapper};
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};
use crate::module_bindings::*;


pub fn attatch_objects(
    player_query: Query<&Transform, With<Player>>,
    mut objects_query: Query<(&PlayerAttach, &mut Transform), Without<Player>>,
    ctx_wrapper: Res<CtxWrapper>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            // Calculate the rotated offset
            let rotated_offset = player_transform.rotation * Vec3::new(attach.offset.x, attach.offset.y, 5.0);
            
            // Update position and rotation
            transform.translation = player_transform.translation + rotated_offset;
            transform.rotation = player_transform.rotation;

            let x = transform.translation.x;
            let y = transform.translation.y;
            let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
            
            if let Some(player) = ctx_wrapper.ctx.db.player().identity().find(&ctx_wrapper.ctx.identity()) {
                ctx_wrapper.ctx.reducers().update_hook_position(
                    ctx_wrapper.ctx.identity(),
                    vec_2_type::Vec2 { x: x, y: y },
                    rotation,
                    player.hook.width,
                    player.hook.height,
                ).unwrap();
            }
        }
    }
}