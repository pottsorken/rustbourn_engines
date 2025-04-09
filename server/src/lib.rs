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

#[spacetimedb::table(name = bots, public)]
pub struct Bot {
    #[primary_key]
    id: u64,
    position: BevyTransform,
    alive: bool, // instead of online we have alive that checks if that specific bot is alive
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

// Reducer for updating player position
#[spacetimedb::reducer]
pub fn update_bot_position(
    ctx: &ReducerContext,
    bevy_transform: BevyTransform,
    bot_id: u64,
) -> Result<(), String> {
    log::info!(
        "Code reaches this point! --------{:?}------",
        bevy_transform
    );
    if let Some(mut _bot) = ctx.db.bots().iter().find(|b| b.id == bot_id) {
        _bot.position = bevy_transform;

        ctx.db.bots().id().update(_bot);

        Ok(())
    } else {
        Err("Bot not found".to_string())
    }
}

#[spacetimedb::reducer]
pub fn reset_bots_if_no_players_online(ctx: &ReducerContext) -> Result<(), String> {
    // Check if any players are online
    let online_players_exist = ctx.db.player().iter().any(|p| p.online);

    if online_players_exist {
        return Ok(()); // Do nothing if any player is online
    }

    // Your original bot spawn points
    let bot_spawn_positions = vec![(200.0, 20.0), (-200.0, -20.0), (200.0, -250.0)];

    // Reset each bot (e.g., set them to some default positions)
    for (i, mut bot) in ctx.db.bots().iter().enumerate() {
        let (x, y) = bot_spawn_positions.get(i).cloned().unwrap_or((0.0, 0.0));

        bot.position = BevyTransform {
            coordinates: Vec2 { x, y },
            rotation: 0.0,
            scale: Vec2 { x: 0.0, y: 0.0 },
        };
        ctx.db.bots().id().update(bot);
    }

    Ok(())
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

    reset_bots_if_no_players_online(ctx);
}

#[spacetimedb::reducer(init)]
pub fn server_startup(ctx: &ReducerContext) {
    generate_obstacles(ctx);
    generate_bots(ctx);
}

fn generate_bots(ctx: &ReducerContext) {
    // Example bot generation logic
    let bot_spawn_positions = vec![(200.0, 20.0), (-200.0, -20.0), (200.0, -250.0)];

    // Generate and insert bots into the database
    for (i, (x, y)) in bot_spawn_positions.into_iter().enumerate() {
        let bot_id = i as u64; // Unique ID for each bot
        let bot_transform = BevyTransform {
            coordinates: Vec2 { x, y },
            rotation: 0.0, // Initial rotation
            scale: Vec2 { x: 1.0, y: 1.0 },
        };

        // Insert bot into the database
        ctx.db.bots().insert(Bot {
            id: bot_id,
            position: bot_transform,
            alive: true, // Bots are alive when generated
        });
    }
}

fn generate_obstacles(ctx: &ReducerContext) {
    let perlin_x = Perlin::new(21);
    let perlin_y = Perlin::new(1345);
    // Generate 1000 obstacles
    for i in 0..200 {
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
