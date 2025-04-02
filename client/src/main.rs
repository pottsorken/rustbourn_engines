use bevy::prelude::*;

mod module_bindings;
use module_bindings::*;
use spacetimedb_sdk::{credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey};

/// The URI of the SpacetimeDB instance hosting our chat module.
const HOST: &str = "http://localhost:3000";

/// The database name we chose when we published our module.
const DB_NAME: &str = "test";

fn main(){
    let ctx = connect_to_db();

    // Construct a new position
    let position = BevyTransform {
        coordinates: Vec2 { x: 2.0, y: 3.0 },
        rotation: 20.0,
        scale: Vec2 { x: 70.0, y: 80.0 },
    };

    //Call update_player_position and check for errors
    match ctx.reducers.update_player_position(position) {
        Ok(_) => println!("Player position updated successfully"),
        Err(e) => eprintln!("Failed to update player position: {:?}", e),
    }

    loop{
        //do nothing
    }
}

fn connect_to_db() -> DbConnection {
    match DbConnection::builder()
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
