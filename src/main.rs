use audio::AudioPlugin;
use bevy::{prelude::*, window::WindowMode};

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera_rendering::CameraRendering;
use game_mechanics::GameMechanicsPlugin;
use level::LevelPlugin;
use object_rendering::ObjectRenderingPlugin;
use text_display::TextDisplayPlugin;

mod audio;
mod camera_rendering;
mod game_mechanics;
mod level;
mod object_rendering;
mod text_display;

fn main() {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Windowed,
            title: "RGB".to_owned(),
            ..default()
        }),
        ..default()
    });

    #[cfg(debug_assertions)]
    let default_plugins = default_plugins.set(AssetPlugin {
        watch_for_changes: true,
        ..default()
    });

    app.add_plugins(default_plugins);

    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_plugin(AudioPlugin)
        .add_plugin(CameraRendering)
        .add_plugin(GameMechanicsPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(ObjectRenderingPlugin)
        .add_plugin(TextDisplayPlugin)
        .run();
}
