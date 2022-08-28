use bevy::{asset::AssetServerSettings, prelude::*, window::WindowMode};

use bevy_inspector_egui::WorldInspectorPlugin;
use camera_rendering::CameraRendering;
use game_mechanics::GameMechanicsPlugin;
use level::LevelPlugin;
use object_rendering::ObjectRenderingPlugin;
use text_display::TextDisplayPlugin;

mod camera_rendering;
mod game_mechanics;
mod level;
mod object_rendering;
mod text_display;

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(WindowDescriptor {
            mode: WindowMode::Windowed,
            // mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GameMechanicsPlugin)
        .add_plugin(ObjectRenderingPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(TextDisplayPlugin)
        .add_plugin(CameraRendering)
        .run();
}
