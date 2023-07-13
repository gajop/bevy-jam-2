use std::time::Duration;

use audio::AudioPlugin;
use bevy::{prelude::*, window::WindowMode, asset::ChangeWatcher};

// #[cfg(debug_assertions)]
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
        ..default()
    });

    app.add_plugins(default_plugins);

    // TODO: Add after bevy-inspector-egui is ported to bevy 0.11
    // #[cfg(debug_assertions)]
    // app.add_plugins(WorldInspectorPlugin::new());

    app.add_plugins(AudioPlugin)
        .add_plugins(CameraRendering)
        .add_plugins(GameMechanicsPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(ObjectRenderingPlugin)
        .add_plugins(TextDisplayPlugin)
        .run();
}
