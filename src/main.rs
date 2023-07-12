mod plugins;
mod setup;
use bevy::prelude::*;
use plugins::{CamMovementPlugin, SelectionPlugin};
use setup::SetupPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            230. / 255.,
            230. / 255.,
            250. / 255.,
        )))
        .insert_resource(AmbientLight {
            brightness: 2f32,
            color: Color::WHITE,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (800f32, 500f32).into(),
                // Tells wasm to resize the window according to the available canvas
                position: WindowPosition::Automatic,
                focused: true,
                // resizable: false,
                // mode: bevy::window::WindowMode::Fullscreen,
                decorations: false,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,

                ..default()
            }),
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        }))
        .add_plugin(CamMovementPlugin)
        .add_plugin(SetupPlugin)
        .add_plugin(SelectionPlugin)
        .add_system(bevy::window::close_on_esc)
        // .add_startup_system(print_resources)
        .run()
}

fn print_resources(world: &World) {
    let components = world.components();

    let mut r: Vec<_> = world
        .storages()
        .resources
        .iter()
        .map(|(id, _)| components.get_info(id).unwrap())
        .map(|info| info.name())
        .collect();

    // sort list alphebetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}
