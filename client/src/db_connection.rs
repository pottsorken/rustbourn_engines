use bevy::prelude::*;

// Spacetime dependencies
use crate::module_bindings::*;
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

/// The URI of the SpacetimeDB instance hosting our chat module.
const HOST: &str = "http://localhost:3000";

/// The database name we chose when we published our module.
const DB_NAME: &str = "test";

pub fn update_player_position(ctx: &DbConnection, player_transform: &Transform) {
    ctx.reducers()
        .update_player_position(BevyTransform {
            coordinates: vec_2_type::Vec2 {
                x: player_transform.translation.x,
                y: player_transform.translation.y,
            },
            rotation: 0.0,
            scale: vec_2_type::Vec2 { x: 0.0, y: 0.0 },
        })
        .unwrap();
}

pub fn setup_connection(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Connect to the database
    let ctx = connect_to_db();

    //// Register callbacks to run in re\sponse to database events
    //register_callbacks(&ctx);
    //
    //// Subscribe to SQL queries in order to construct a local partial replica of the database
    //subscribe_to_tables(&ctx);
    //
    // Spawn a thread, where the connection will process messages and invoke callbacks.
    ctx.run_threaded();

    commands.insert_resource(ctx);
    // Handle CLI input
    //user_input_loop(&ctx);
}

fn connect_to_db() -> DbConnection {
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
        .with_uri(HOST)
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
