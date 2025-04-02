use bevy::prelude::*;

mod module_bindings;
use module_bindings::*;
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

/// The URI of the SpacetimeDB instance hosting our chat module.
const HOST: &str = "http://localhost:3000";

/// The database name we chose when we published our module.
const DB_NAME: &str = "test";

fn main() {
    let ctx = connect_to_db();

    ctx.run_threaded();
    // Construct a new position
    let position = BevyTransform {
        coordinates: vec_2_type::Vec2 { x: 2.0, y: 3.0 },
        rotation: 20.0,
        scale: vec_2_type::Vec2 { x: 70.0, y: 80.0 },
    };

    println!("position: {:?}", position);
    //Call update_player_position and check for errors
    match ctx.reducers.update_player_position(position) {
        Ok(_) => println!("Player position updated successfully"),
        Err(e) => eprintln!("Failed to update player position: {:?}", e),
    }

    loop {
        //do nothing
    }
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
