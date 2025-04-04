use spacetimedb::{
    reducer, spacetimedb_lib::db, table, DbContext, Identity, ReducerContext, SpacetimeType, Table,
    Timestamp,
};

use noise::{NoiseFn, Perlin};

// Player data
#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
    position: BevyTransform,
    online: bool,
}

#[spacetimedb::table(name = obstacle, public)]
pub struct Obstacle {
    position: Vec2,
    id: u64,
}

// Bevy transform data
#[derive(Debug, SpacetimeType)]
pub struct BevyTransform {
    coordinates: Vec2,
    rotation: f32,
    scale: Vec2,
}

// Vector with x, y coordinates
#[derive(Debug, SpacetimeType)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

// Reducer for updating player position
#[spacetimedb::reducer]
pub fn update_player_position(
    ctx: &ReducerContext,
    bevy_transform: BevyTransform,
) -> Result<(), String> {
    log::info!(
        "Code reaches this point! --------{:?}------",
        bevy_transform
    );
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            position: bevy_transform,
            .._player
        });
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}

// Reducer for creating and/or login existing player to server
// Called when client connects
#[spacetimedb::reducer(client_connected)]
pub fn player_connected(ctx: &ReducerContext) {
    log::info!(" HEY -----");
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            online: true,
            .._player
        });
    } else {
        ctx.db.player().insert(Player {
            identity: ctx.sender,
            position: BevyTransform {
                coordinates: Vec2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                scale: Vec2 { x: 50.0, y: 100.0 },
            },
            online: true,
        });
    }
}

// Reducer for logging out player
// Called when client disconnects from server
#[spacetimedb::reducer(client_disconnected)]
pub fn player_disconnected(ctx: &ReducerContext) {
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            online: false,
            .._player
        });
    } else {
        // Should never reach!!
        log::warn!(
            "Disconnect event for unknown user with identity {:?}",
            ctx.sender
        )
    }
}

#[spacetimedb::reducer(init)]
pub fn server_startup(ctx: &ReducerContext) {
    generate_obstacles(ctx);
}

fn generate_obstacles(ctx: &ReducerContext) {
    let perlin_x = Perlin::new(21);
    let perlin_y = Perlin::new(1345);
    // Generate 1000 obstacles
    for i in 0..1000 {
        let x = (i as f32) / 10.0; // Control frequency
        let y = ((i + 1) as f32) / 10.0;

        let random_x = perlin_x.get([x as f64, y as f64]) as f32 * 4056.0;
        let random_y = perlin_y.get([y as f64, x as f64]) as f32 * 4056.0;
        let invalid_x = random_x < 300.0 && random_x > -300.0;
        let invalid_y = random_y < 300.0 && random_y > -300.0;

        if invalid_x && invalid_y {
            continue;
        }

        ctx.db.obstacle().insert(Obstacle {
            position: Vec2 {
                x: random_x,
                y: random_y,
            },
            id: i,
        });
    }
}

