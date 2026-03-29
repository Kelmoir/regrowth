use bevy::prelude::*;

mod grid;
mod components;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Regrowth - Ecological Simulation".into(),
                resolution: (1600u32, 900u32).into(),
                ..default()
            }),
            ..default()
        }))
        // Simulation time
        .add_systems(Startup, setup)
        // Plugins will be added here
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
