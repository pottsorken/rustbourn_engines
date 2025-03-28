use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window : Some(Window{
                    title : String::from("Rustbourn Engines"),
                    position : WindowPosition::Centered(MonitorSelection::Primary),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_systems(Startup, setup_map)
        .run();
}

#[derive(Component)]

struct Base{
    pub velocity : f32,
}

fn setup_map(
    mut commands : Commands
){
    commands.insert_resource(ClearColor(Color::srgb(0.37, 0.5, 0.0))); // rgb(37.6% 50.7% 0%)

    commands.spawn(Camera2d::default());
}