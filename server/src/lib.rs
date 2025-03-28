use spacetimedb::{reducer, spacetimedb_lib::db, table, Identity, ReducerContext, Table, Timestamp, SpacetimeType};

// Player data
#[spacetimedb::table(name = player, public)]
pub struct Player{
    #[primary_key]
    identity: Identity,
    position: BevyTransform,
    online: bool,
}

// Bevy transform data
#[derive(SpacetimeType)]
pub struct
BevyTransform{
    coordinates: Vec2,
    rotation: f32,
    scale: Vec2,
}

// Vector with x, y coordinates
#[derive(SpacetimeType)]
pub struct Vec2{
    x: f32,
    y: f32,
}

// Reducer for updating player position
#[spacetimedb::reducer]
pub fn update_player_position(ctx: &ReducerContext, bevy_transform: BevyTransform) -> Result<(), String>{
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender){
        ctx.db.player().identity().update(Player{
            position: bevy_transform,
            .._player
        });
        Ok(())
    } else{
        Err("Player not found".to_string())
    }
}

// Reducer for creating and/or login existing player to server
// Called when client connects
#[spacetimedb::reducer(client_connected)]
pub fn player_connected(ctx: &ReducerContext){
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender){
        ctx.db.player().identity().update(Player{online: true, .._player});
    } else{
        ctx.db.player().insert(Player{
            identity: ctx.sender,
            position: BevyTransform {
                coordinates: Vec2{x: 0.0, y: 0.0},
                rotation: 0.0,
                scale: Vec2{x: 50.0, y: 100.0},
            },
            online: true,
        });
    }
}

// Reducer for logging out player
// Called when client disconnects from server
#[spacetimedb::reducer(client_disconnected)]
pub fn player_disconnected(ctx: &ReducerContext){
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender){
        ctx.db.player().identity().update(Player{online: false, .._player});
    } else{
        // Should never reach!!
        log::warn!("Disconnect event for unknown user with identity {:?}", ctx.sender)
    }
}
