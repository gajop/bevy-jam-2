use bevy::{prelude::*, render::view::RenderLayers};

pub struct TextDisplayPlugin;

impl Plugin for TextDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_text);
    }
}

#[derive(Component)]
struct TimerText;

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Time: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,

                position: UiRect {
                    left: Val::Percent(50.0),
                    bottom: Val::Percent(0.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(TimerText);

    commands.spawn_bundle(
        TextBundle::from_section(
            "Controls: WASD for movement
			Reach the goal (ring) without hitting any walls",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::CENTER_RIGHT)
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,

            position: UiRect {
                bottom: Val::Percent(0.0),
                left: Val::Percent(0.0),
                ..default()
            },
            ..default()
        }),
    );
}
