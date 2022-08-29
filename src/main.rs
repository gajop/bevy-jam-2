use audio::AudioPlugin;
use bevy::{prelude::*, window::WindowMode};

#[cfg(debug_assertions)]
use bevy::asset::AssetServerSettings;
#[cfg(debug_assertions)]
use bevy_inspector_egui::WorldInspectorPlugin;

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

    #[cfg(debug_assertions)]
    app.insert_resource(AssetServerSettings {
        watch_for_changes: true,
        ..default()
    });

    app.insert_resource(WindowDescriptor {
        mode: WindowMode::Windowed,
        // mode: WindowMode::BorderlessFullscreen,
        ..default()
    })
    .add_plugins(DefaultPlugins);

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
