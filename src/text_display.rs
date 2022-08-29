use bevy::prelude::*;

use crate::{game_mechanics::GameTimer, level::LevelInfo};

pub struct TextDisplayPlugin;

impl Plugin for TextDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_text)
            .add_system(win_text)
            .add_system(text_update_system);
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
                    left: Val::Percent(45.0),
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
                top: Val::Percent(0.0),
                right: Val::Percent(0.0),
                ..default()
            },
            ..default()
        }),
    );
}

fn win_text(mut commands: Commands, asset_server: Res<AssetServer>, level_info: Res<LevelInfo>) {
    if !level_info.is_changed() {
        return;
    }

    if level_info.index != Some(level_info.total_levels as i32) {
        return;
    }
    commands.spawn_bundle(
        TextBundle::from_section(
            "Congratulations! You win

            Reload to play again",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 35.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::CENTER_RIGHT)
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,

            position: UiRect {
                top: Val::Percent(40.0),
                left: Val::Percent(40.0),
                ..default()
            },
            ..default()
        }),
    );
}

fn text_update_system(timer: Res<GameTimer>, mut query: Query<&mut Text, With<TimerText>>) {
    let mut one_second = false;
    let mut two_seconds = false;
    let timer_text = match &timer.0 {
        Some(timer) => {
            let remaining = (timer.duration().as_millis() as f32 / 1000.0) - timer.elapsed_secs();
            if remaining <= 1.0 {
                one_second = true;
            } else if remaining <= 2.0 {
                two_seconds = true;
            }
            format!("Timer: {:.0}", remaining + 0.5)
        }
        None => "".to_owned(),
    };
    for mut text in &mut query {
        text.sections[0].value = timer_text.clone();

        text.sections[0].style.color = if one_second {
            Color::rgb(1.0, 0.0, 0.0)
        } else if two_seconds {
            Color::rgb(1.0, 1.0, 0.0)
        } else {
            Color::rgb(1.0, 1.0, 1.0)
        }
    }
}
