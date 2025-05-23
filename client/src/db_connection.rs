use bevy::ecs::system::ParamSet;
use bevy::prelude::*;


// Spacetime dependencies
use crate::common::{CtxWrapper, Opponent, OpponentTrack};
use crate::opponent::*;
use crate::{module_bindings::*, player};
use spacetimedb_sdk::{credentials, DbContext, Error, Identity, Table};

use crate::parse::*;

use crate::common::HookAttach;
use crate::common::OpponentHookHead;
use crate::common::{OpponentHook, Username};
use crate::hook::*;

/// The database name we chose when we published our module.
//const DB_NAME: &str = "c200083d815ce43080deb1559d525d655b7799ec50b1552f413b372555053a1c";
pub const DB_NAME: &str = "test";

pub fn update_player_position(ctx_wrapper: &CtxWrapper, player_transform: &Transform) {
    ctx_wrapper
        .ctx
        .reducers()
        .update_player_position(BevyTransform {
            coordinates: vec_2_type::Vec2 {
                x: player_transform.translation.x,
                y: player_transform.translation.y,
            },
            rotation: player_transform.rotation.to_euler(EulerRot::XYZ).2,
            scale: vec_2_type::Vec2 { x: 0.0, y: 0.0 },
        })
        .unwrap();
    //println!("{}", player_transform.rotation.to_euler(EulerRot::XYZ).2);
}

// db_connection
pub fn update_bot_position(
    ctx_wrapper: &CtxWrapper,
    bot_transform: &Transform,
    bot_id: u64,
    new_rotate_dir: f32,
) {
    ctx_wrapper
        .ctx
        .reducers()
        .update_bot_position(
            BevyTransform {
                coordinates: vec_2_type::Vec2 {
                    x: bot_transform.translation.x,
                    y: bot_transform.translation.y,
                },
                rotation: bot_transform.rotation.to_euler(EulerRot::XYZ).2,
                scale: vec_2_type::Vec2 { x: 0.0, y: 0.0 },
            },
            bot_id,
            new_rotate_dir,
        )
        .unwrap();
    //println!("{}", player_transform.rotation.to_euler(EulerRot::XYZ).2);
}

pub fn db_setup() -> DbConnection {
    connect_to_db()
}

pub fn setup_connection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ctx_wrapper: Res<CtxWrapper>,
    username: Res<Username>,
) {
    // Connect to the database
    //let ctx = connect_to_db();

    subscribe_to_tables(&ctx_wrapper.ctx);

    //// Register callbacks to run in re\sponse to database events
    //register_callbacks(&ctx);
    //use crate::common::OpponentHook;
    //// Subscribe to SQL queries in order to construct a local partial replica of the database
    //subscribe_to_tables(&ctx);
    //
    // Spawn a thread, where the connection will process messages and invoke callbacks.
    ctx_wrapper.ctx.run_threaded();

    //commands.insert_resource(CtxWrapper { ctx });
    // Handle CLI input
    //user_input_loop(&ctx);

    // Set username
    ctx_wrapper
        .ctx
        .reducers()
        .set_name(username.into_inner().name.clone());
}

/// Register subscriptions for all rows of the player tables.
fn subscribe_to_tables(ctx: &DbConnection) {
    ctx.subscription_builder()
        .on_applied(on_sub_applied)
        .on_error(on_sub_error)
        .subscribe([
            "SELECT * FROM player WHERE online=true",
            "SELECT * FROM obstacle",
            "SELECT * FROM bots",
            "SELECT * FROM block",
        ]);
}

/// Our `on_subscription_applied` callback:
fn on_sub_applied(ctx: &SubscriptionEventContext) {
    let mut positions = ctx.db.player().iter().collect::<Vec<_>>();
    for position in positions {
        println!("{:?}", position);
    }
    // Forgot why i added this, but it seems to work wihtout
    let bots = ctx.db.bots().iter().collect::<Vec<_>>();
    println!("[DEBUG] Bots in on_sub_applied: {}", bots.len());
}

fn on_sub_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Subscription failed: {}", err);
    std::process::exit(1);
}

fn connect_to_db() -> DbConnection {
    let server_url = parse_args();

    //let server_url = parse_args();
    //println!("Server url: {:?}", server_url.to_string());
    match DbConnection::builder()
        // Register our `on_connect` callback, which will save our auth token.
        .on_connect(on_connected)
        // Register our `on_connect_error` callback, which will print a message, then exit the process.
        .on_connect_error(on_connect_error)
        // Our `on_disconnect` callback, which will print a message, then exit the process.
        .on_disconnect(on_disconnected)
        // If the user has previously connected, we'll have saved a token in the `on_connect` callback.
        // In that case, we'll load it and pass it to `with_token`,
        // so we can re-authenticate as the same `Identity`.
        .with_token(creds_store().load().expect("Error loading credentials"))
        .with_module_name(DB_NAME)
        .with_uri(server_url)
        .build()
    {
        Ok(db) => {
            println!("Connected to database successfully!");
            db
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            std::process::exit(1); // Exit the program gracefully
        }
    }
}

fn creds_store() -> credentials::File {
    credentials::File::new(DB_NAME)
}

/// Our `on_connect` callback: save our credentials to a file.
fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {
    if let Err(e) = creds_store().save(token) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

/// Our `on_connect_error` callback: print the error, then exit the process.
fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Connection error: {:?}", err);
    std::process::exit(1);
}

/// Our `on_disconnect` callback: print a note, then exit the process.
fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
    if let Some(err) = err {
        eprintln!("Disconnected: {}", err);
        std::process::exit(1);
    } else {
        println!("Disconnected.");
        std::process::exit(0);
    }
}

pub fn update_opponent_positions(
    ctx_wrapper: Res<CtxWrapper>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Transform, &Opponent)>,
) {
    let players = ctx_wrapper.ctx.db.player().iter().collect::<Vec<_>>();
    let local_player_id = ctx_wrapper.ctx.identity(); //Get local player's ID

    for player in players {
        let temp_id = player.identity.to_u256() % 10_000;
        if temp_id == 9573 {
            //print!("{}: {}   ", temp_id, player.position.rotation);
        }
        spawn_opponent(
            &mut commands,
            &asset_server,
            &query,
            &player.identity,
            player.position.coordinates.x,
            player.position.coordinates.y,
            player.position.rotation,
            &local_player_id,
        );
        update_opponent(
            &mut query,
            &player.identity,
            player.position.coordinates.x,
            player.position.coordinates.y,
            player.position.rotation,
        );
    }
    //println!("");
}

pub fn load_obstacles(ctx_wrapper: &CtxWrapper) -> Vec<(f32, f32, u64, u32)> {
    let obstacles: Vec<(f32, f32, u64, u32)> = ctx_wrapper
        .ctx
        .db
        .obstacle()
        .iter()
        .map(|obstacle| {
            (
                obstacle.position.x,
                obstacle.position.y,
                obstacle.id,
                obstacle.hp,
            )
        })
        .collect();
    obstacles
}

pub fn load_bots(ctx_wrapper: &CtxWrapper) -> Vec<(f32, f32, u64)> {
    println!("[DEBUG] object)",);
    let bots: Vec<(f32, f32, u64)> = ctx_wrapper
        .ctx
        .db
        .bots()
        .iter()
        .map(|bot| {
            (
                bot.position.coordinates.x,
                bot.position.coordinates.y,
                bot.id,
            )
        })
        .collect();
    bots
}

pub fn load_leaderboard(ctx_wrapper: &CtxWrapper) -> Vec<(Identity, u64)> {
    println!("[DEBUG] Loading leaderboard");

    // Attempt to get the leaderboard with ID 1
    if let Some(leaderboard) = ctx_wrapper.ctx.db.leaderboard().id().find(&1) {
        // Map the top player identities to their position and score
        leaderboard
            .top_players
            .iter()
            .filter_map(|&identity| {
                // Fetch the player by identity
                ctx_wrapper
                    .ctx
                    .db
                    .player()
                    .identity()
                    .find(&identity)
                    .map(|player| {
                        let name = player.identity;
                        let blocks = player.grid.load as u64;
                        (name, blocks) // Return the tuple
                    })
            })
            .collect() // Collect the result into a Vec
    } else {
        println!("[ERROR] Leaderboard not found");
        Vec::new() // Return an empty vector if the leaderboard does not exist
    }
}

// db_connection

//pub fn update_leaderboard(
//    ctx_wrapper: &CtxWrapper,
//    //bot_transform: &Transform,
//    player_id: Identity,
//    leaderboard_id: u64,
//    //new_rotate_dir: f32,
//) {
//    ctx_wrapper
//        .ctx
//        .reducers()
//        .update_leaderboard(
//            player_id,
//            leaderboard_id
//
//        )
//        .unwrap();
//}

pub fn update_opponent_hooks(
    ctx_wrapper: Res<CtxWrapper>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut query: Query<(&mut Sprite, &mut Transform, &OpponentHook), With<OpponentHook>>,
    /*mut hook_query: Query<(&mut Sprite, &mut Transform, &OpponentHook), With<OpponentHook>>,
     mut head_query: Query<(&mut Transform, &HookAttach, &OpponentHookHead)>,
    */
    mut queries: ParamSet<(
        Query<(&mut Sprite, &mut Transform, &OpponentHook), With<OpponentHook>>,
        Query<(&mut Transform, &HookAttach, &OpponentHookHead)>,
    )>,
    existing_hooks_query: Query<&OpponentHook>,
    existing_heads_query: Query<&OpponentHookHead>,
    despawn_query: Query<(Entity, &OpponentHook)>,
) {
    let players = ctx_wrapper.ctx.db.player().iter().collect::<Vec<_>>();

    let local_player_id = ctx_wrapper.ctx.identity(); //Get local player's ID

    for player in players {
        let player_id = player.identity;
        spawn_opponent_hook(
            &mut commands,
            &asset_server,
            &existing_hooks_query,
            &existing_heads_query,
            &player_id,
            &local_player_id,
            player.hook.position.x,
            player.hook.position.y,
        );

        update_opponent_hook(
            /*&mut hook_query,
            &mut head_query,*/
            &mut queries,
            &player_id,
            player.hook.position.x,
            player.hook.position.y,
            player.hook.rotation,
            player.hook.width,
            player.hook.height,
        );
    }
}

pub fn update_opponent_tracks(
    ctx_wrapper: Res<CtxWrapper>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Sprite, &mut Transform, &OpponentTrack), With<OpponentTrack>>,
    existing_tracks_query: Query<&OpponentTrack>,
) {
    let players = ctx_wrapper.ctx.db.player().iter().collect::<Vec<_>>();

    let local_player_id = ctx_wrapper.ctx.identity(); //Get local player's ID

    for player in players {
        let player_id = player.identity;
        spawn_opponent_track(
            &mut commands,
            &asset_server,
            &existing_tracks_query,
            &player_id,
            &local_player_id,
            player.track.id,
            player.track.position.coordinates.x,
            player.track.position.coordinates.y,
            player.track.rotation,
            player.track.width,
            player.track.height,
        );

        update_opponent_track(
            &mut query,
            player.track.id,
            player.track.position.coordinates.x,
            player.track.position.coordinates.y,
            player.track.rotation,
            player.track.width,
            player.track.height,
        );
    }
}


pub fn despawn_opponent_hooks(
    ctx_wrapper: Res<CtxWrapper>,
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(Entity, &OpponentHook)>,
        Query<(Entity, &OpponentHookHead)>,
    )>,
) {
crate::hook::despawn_opponent_hook_entities(commands, ctx_wrapper, queries);
}

